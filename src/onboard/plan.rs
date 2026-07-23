use crate::onboard::{OnboardOptions, REQUIRED_ACTIONS};

/// Render the fully offline `--dry-run` plan: every call the command would
/// make and the complete CreateStack parameter set, with placeholders for
/// values fetched at run time. The AuthToken is never rendered.
pub fn render(opts: &OnboardOptions) -> String {
    let service_account = opts
        .service_account_id
        .clone()
        .unwrap_or_else(|| "<resolved from Plerion API or --service-account-id>".to_string());
    let auto_update = if opts.auto_update { "true" } else { "false" };
    format!(
        "plerion integrations add aws — dry run (no network calls were made)\n\
         \n\
         Plerion API calls (Bearer-authenticated, tenant {tenant}):\n\
         \x20 1. GET  /v1/tenant/external-id\n\
         \x20 2. GET  /v1/tenant/cloudformation-templates?type=AWSAccount\n\
         \x20 3. POST /v1/tenant/integrations/token          (after confirmation; short-lived)\n\
         \n\
         AWS calls (deployer credentials):\n\
         \x20 preflight: sts:GetCallerIdentity, iam:ListAccountAliases (best-effort),\n\
         \x20            cloudformation:DescribeStacks, iam:ListRoles,\n\
         \x20            iam:GetRole + iam:SimulatePrincipalPolicy (advisory, {n} actions),\n\
         \x20            cloudformation:GetTemplateSummary\n\
         \x20 deploy:    cloudformation:CreateStack (CAPABILITY_NAMED_IAM, OnFailure=ROLLBACK),\n\
         \x20            cloudformation:DescribeStacks (poll), DescribeStackEvents (on failure)\n\
         \n\
         CreateStack parameters:\n\
         \x20 StackName             {stack}\n\
         \x20 TemplateURL           <fetched from step 2>\n\
         \x20 PlerionURL            {plerion_url}\n\
         \x20 PlerionAccountId      {plerion_account}\n\
         \x20 ExternalId            <fetched from step 1>\n\
         \x20 AuthToken             <redacted — short-lived, NoEcho in the template>\n\
         \x20 Capabilities          ALL\n\
         \x20 WorkloadScanningType  PlerionManagedServiceAccount\n\
         \x20 ServiceAccountId      {service_account}\n\
         \x20 KMSKeyAccessMode      {kms}\n\
         \x20 EnableAutoUpdate      {auto_update}\n\
         \n\
         Equivalent AWS CLI deploy step:\n\
         \x20 aws cloudformation create-stack --stack-name {stack} \\\n\
         \x20   --template-url <templateURL> --capabilities CAPABILITY_NAMED_IAM \\\n\
         \x20   --parameters ParameterKey=PlerionURL,ParameterValue={plerion_url} ...\n",
        tenant = opts.tenant_label,
        n = REQUIRED_ACTIONS.len(),
        stack = opts.stack_name,
        plerion_url = opts.plerion_url,
        plerion_account = opts.plerion_account_id,
        service_account = service_account,
        kms = opts.kms_key_access_mode,
        auto_update = auto_update,
    )
}
