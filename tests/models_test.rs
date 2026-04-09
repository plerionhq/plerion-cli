use plerion::output::TableRenderable;
use plerion::api::models::*;

#[test]
fn test_tenant_table_renderable() {
    let t = tenant::TenantData {
        tenant_id: "tid".to_string(),
        organization_id: "oid".to_string(),
        name: "My Tenant".to_string(),
        created_at: Some("2025-01-01".to_string()),
        updated_at: None,
        risk_score: Some(8.5),
    };
    let headers = tenant::TenantData::headers();
    assert_eq!(headers.len(), 6);
    assert_eq!(headers[0], "TENANT ID");
    let row = t.row();
    assert_eq!(row[0], "tid");
    assert_eq!(row[2], "My Tenant");
    assert_eq!(row[3], "8.50");
}

#[test]
fn test_tenant_no_risk_score() {
    let t = tenant::TenantData {
        tenant_id: "tid".to_string(),
        organization_id: "oid".to_string(),
        name: "X".to_string(),
        created_at: None,
        updated_at: None,
        risk_score: None,
    };
    let row = t.row();
    assert_eq!(row[3], ""); // empty when no risk score
    assert_eq!(row[4], ""); // empty when no created_at
}

#[test]
fn test_finding_table_renderable() {
    let f = findings::Finding {
        id: Some("prn:123".to_string()),
        schema_version: None,
        organization_id: None,
        tenant_id: None,
        integration_id: None,
        provider: Some("AWS".to_string()),
        execution_id: None,
        asset_id: None,
        provider_account_id: None,
        resource_type: Some("AWS::S3::Bucket".to_string()),
        detection_id: Some("PLERION-AWS-16".to_string()),
        status: Some("FAILED".to_string()),
        severity_level: Some("CRITICAL".to_string()),
        message: None,
        first_observed_at: Some("2023-01-01".to_string()),
        created_at: None,
        last_observed_at: None,
        updated_at: None,
        parameters: None,
        tags: None,
        full_resource_name: None,
        resource_id: None,
        provider_full_resource_name: None,
        region: Some("us-east-1".to_string()),
        service: None,
        likelihood: None,
        impact: None,
        calculated_severity: None,
        modified_severity_level: None,
        attack_paths: None,
        is_exempted: None,
        meta: None,
        resource_tags: None,
        resource_url: None,
    };
    let headers = findings::Finding::headers();
    assert_eq!(headers.len(), 27);
    let row = f.row();
    assert_eq!(row[0], "prn:123");         // id
    assert_eq!(row[1], "PLERION-AWS-16");  // detection_id
    assert_eq!(row[2], "FAILED");          // status
    assert_eq!(row[3], "CRITICAL");        // severity_level
}

#[test]
fn test_finding_with_nones() {
    let f = findings::Finding {
        id: None, schema_version: None, organization_id: None, tenant_id: None,
        integration_id: None, provider: None, execution_id: None, asset_id: None,
        provider_account_id: None, resource_type: None, detection_id: None,
        status: None, severity_level: None, message: None, first_observed_at: None,
        created_at: None, last_observed_at: None, updated_at: None, parameters: None,
        tags: None, full_resource_name: None, resource_id: None,
        provider_full_resource_name: None, region: None, service: None,
        likelihood: None, impact: None, calculated_severity: None,
        modified_severity_level: None, attack_paths: None, is_exempted: None,
        meta: None, resource_tags: None, resource_url: None,
    };
    let row = f.row();
    for cell in &row {
        assert!(cell.is_empty() || cell.contains(""));
    }
}

#[test]
fn test_asset_table_renderable() {
    let a = assets::Asset {
        id: Some("prn:assets:verylongidthatismorethan40characterslongfortruncation".to_string()),
        schema_version: None,
        organization_id: None,
        tenant_id: None,
        integration_id: None,
        execution_id: None,
        provider: Some("AWS".to_string()),
        asset_type: None,
        name: Some("my-instance".to_string()),
        created_at: None,
        first_observed_at: None,
        last_observed_at: None,
        updated_at: None,
        tags: None,
        is_publicly_exposed: Some(true),
        is_vulnerable: Some(false),
        number_of_low_vulnerabilities: None,
        number_of_medium_vulnerabilities: None,
        number_of_high_vulnerabilities: None,
        number_of_critical_vulnerabilities: None,
        vulnerability_score: None,
        has_kev: None,
        has_exploit: None,
        is_exploitable: None,
        is_in_vpc: None,
        last_scan_id: None,
        last_scanned_at: None,
        image_id: None,
        platform: None,
        has_admin_privileges: None,
        has_overly_permissive_privileges: None,
        has_authorizer: None,
        has_tracing_enabled: None,
        policy: None,
        number_of_low_secrets: None,
        number_of_medium_secrets: None,
        number_of_high_secrets: None,
        number_of_critical_secrets: None,
        low_secrets: None,
        medium_secrets: None,
        high_secrets: None,
        critical_secrets: None,
        operating_system: None,
        risk_score: Some(serde_json::json!(9.36)),
        operational_state: Some("active".to_string()),
        region: Some("us-east-1".to_string()),
        service: None,
        resource_id: None,
        resource_name: None,
        resource_tags: None,
        resource_type: Some("AWS::EC2::Instance".to_string()),
        full_resource_name: None,
        provider_account_id: None,
        resource_url: None,
    };
    let row = a.row();
    assert!(row[0].starts_with("prn:")); // full ID
    assert_eq!(row[1], "my-instance");
    assert_eq!(row[10], "9.36");  // risk score (index 10 in new headers)
    assert_eq!(row[12], "yes");   // publicly exposed (index 12)
    assert_eq!(row[13], "no");   // vulnerable (index 13)
}

#[test]
fn test_asset_string_risk_score() {
    let a = assets::Asset {
        id: Some("short-id".to_string()),
        schema_version: None, organization_id: None, tenant_id: None,
        integration_id: None, execution_id: None,
        provider: None, asset_type: None, name: None,
        created_at: None, first_observed_at: None, last_observed_at: None, updated_at: None,
        tags: None,
        is_publicly_exposed: None, is_vulnerable: None,
        number_of_low_vulnerabilities: None, number_of_medium_vulnerabilities: None,
        number_of_high_vulnerabilities: None, number_of_critical_vulnerabilities: None,
        vulnerability_score: None, has_kev: None, has_exploit: None, is_exploitable: None,
        is_in_vpc: None, last_scan_id: None, last_scanned_at: None, image_id: None,
        platform: None, has_admin_privileges: None, has_overly_permissive_privileges: None,
        has_authorizer: None, has_tracing_enabled: None, policy: None,
        number_of_low_secrets: None, number_of_medium_secrets: None,
        number_of_high_secrets: None, number_of_critical_secrets: None,
        low_secrets: None, medium_secrets: None, high_secrets: None, critical_secrets: None,
        operating_system: None,
        risk_score: Some(serde_json::json!("7.5")),
        operational_state: None, region: None, service: None,
        resource_id: None, resource_name: None, resource_tags: None,
        resource_type: None, full_resource_name: None,
        provider_account_id: None, resource_url: None,
    };
    let row = a.row();
    assert_eq!(row[0], "short-id");
    assert_eq!(row[10], "7.5"); // string risk score (index 10 in new headers)
}

#[test]
fn test_alert_table_renderable() {
    let a = alerts::Alert {
        id: Some("prn:alerts:abcdef01234567890123456789012345678901".to_string()),
        tenant_id: None,
        integration_id: None,
        status: Some("OPEN".to_string()),
        flagged: Some(true),
        acknowledged: None,
        workflow_id: None,
        title: Some("High risk asset".to_string()),
        created_at: Some("2025-01-01".to_string()),
        updated_at: None,
        summary: None,
        alert_type: Some("ASSET".to_string()),
        risk_score: Some(9.5),
        discovered_date: None,
        last_scanned_at_timestamp: None,
        rules_changed_at_timestamp: None,
        closed_at_timestamp: None,
        meta: None,
    };
    let row = a.row();
    assert_eq!(row[1], "High risk asset");
    assert_eq!(row[3], "ASSET");
    assert_eq!(row[4], "9.50");
    assert_eq!(row[5], "yes");
}

#[test]
fn test_risk_table_renderable() {
    let r = risks::Risk {
        id: Some("risk-1".to_string()),
        risk_type_id: Some("PLERION-RISK-1".to_string()),
        organization_id: None,
        tenant_id: None,
        integration_id: None,
        description: None,
        primary_asset_id: None,
        region: Some("us-east-1".to_string()),
        resolutions: None,
        score: Some(9.5),
        likelihood: None,
        impact: None,
        severity_level: Some("CRITICAL".to_string()),
        factors: None,
        meta: None,
        discovered_at: Some("2025-01-01".to_string()),
        lifecycle_state: Some("OPEN".to_string()),
    };
    let headers = risks::Risk::headers();
    assert_eq!(headers.len(), 17);
    let row = r.row();
    assert_eq!(row[0], "risk-1");
    assert_eq!(row[3], "9.50");
}

#[test]
fn test_vulnerability_table_renderable() {
    let v = vulnerabilities::Vulnerability {
        schema_version: None,
        asset_id: None,
        organization_id: None,
        tenant_id: None,
        integration_id: None,
        vulnerability_id: Some("CVE-2023-1234".to_string()),
        provider: Some("AWS".to_string()),
        asset_type: Some("AWS::EC2::Instance".to_string()),
        description: None,
        severity_level: Some("HIGH".to_string()),
        first_observed_at: Some("2023-10-27".to_string()),
        last_observed_at: None,
        published_date: None,
        execution_id: None,
        title: Some("A very long vulnerability title that exceeds fifty characters to test truncation".to_string()),
        target_name: None,
        severity_source: None,
        primary_url: None,
        packages: None,
        cwes: None,
        has_kev: Some(true),
        has_exploit: Some(false),
        has_vendor_fix: Some(true),
        known_exploit: None,
        exploits: None,
        exemptions: None,
        severity_level_value: None,
    };
    let row = v.row();
    assert_eq!(row[0], "CVE-2023-1234");
    // Title is now full length (no truncation)
    assert!(row[1].contains("very long vulnerability title"));
    assert_eq!(row[11], "yes"); // KEV (index 11 in new headers)
    assert_eq!(row[12], "no");  // exploit (index 12)
    assert_eq!(row[13], "yes"); // fix (index 13)
}

#[test]
fn test_vulnerability_exemption_table_renderable() {
    let e = vulnerabilities::VulnerabilityExemption {
        id: Some("ex-1".to_string()),
        profile_id: None,
        name: Some("My exemption".to_string()),
        audit_note: None,
        reason: Some("ACCEPTED_RISK".to_string()),
        conditions: None,
        created_at: Some("2025-01-01".to_string()),
        updated_at: None,
        created_by: None,
        updated_by: None,
    };
    let row = e.row();
    assert_eq!(row[0], "ex-1");
    assert_eq!(row[2], "My exemption");   // NAME is at index 2 now
    assert_eq!(row[3], "ACCEPTED_RISK");  // REASON is at index 3 now
}

#[test]
fn test_integration_table_renderable() {
    let i = integrations::Integration {
        integration_id: Some("int-1".to_string()),
        name: Some("AWS Prod".to_string()),
        provider: Some("AWS".to_string()),
        integration_type: Some("AWSAccount".to_string()),
        status: Some("Active".to_string()),
        risk_score: Some(8.19),
        tenant_id: None,
        organization_id: None,
        schedule: None,
        scan_interval: None,
        detection_setting_id: None,
        created_at: None,
        updated_at: None,
        aws_account_id: Some("123456789012".to_string()),
        azure_subscription_id: None,
        azure_directory_id: None,
        gcp_project_id: None,
    };
    let row = i.row();
    assert_eq!(row[0], "int-1");
    assert_eq!(row[1], "AWS Prod");
    assert_eq!(row[11], "123456789012"); // aws_account_id (index 11 in new headers)
}

#[test]
fn test_integration_azure_account() {
    let i = integrations::Integration {
        integration_id: None, name: None, provider: Some("Azure".to_string()),
        integration_type: None, status: None, risk_score: None,
        tenant_id: None, organization_id: None,
        schedule: None, scan_interval: None, detection_setting_id: None,
        created_at: None, updated_at: None,
        aws_account_id: None,
        azure_subscription_id: Some("sub-123".to_string()),
        azure_directory_id: None,
        gcp_project_id: None,
    };
    let row = i.row();
    assert_eq!(row[12], "sub-123"); // azure_subscription_id (index 12 in new headers)
}

#[test]
fn test_integration_gcp_project() {
    let i = integrations::Integration {
        integration_id: None, name: None, provider: Some("GCP".to_string()),
        integration_type: None, status: None, risk_score: None,
        tenant_id: None, organization_id: None,
        schedule: None, scan_interval: None, detection_setting_id: None,
        created_at: None, updated_at: None,
        aws_account_id: None, azure_subscription_id: None,
        azure_directory_id: None,
        gcp_project_id: Some("my-project".to_string()),
    };
    let row = i.row();
    assert_eq!(row[14], "my-project"); // gcp_project_id (index 14 in new headers)
}

#[test]
fn test_asset_group_table_renderable() {
    let ag = asset_groups::AssetGroup {
        asset_group_id: Some("ag-1".to_string()),
        name: Some("Production".to_string()),
        status: Some("completed".to_string()),
        total_assets: Some(42),
        risk_score: Some(7.5),
        tenant_id: None,
        organization_id: None,
        created_at: Some("2025-01-01".to_string()),
        updated_at: None,
    };
    let row = ag.row();
    assert_eq!(row[0], "ag-1");
    assert_eq!(row[1], "Production");
    assert_eq!(row[3], "42");
    assert_eq!(row[4], "7.50");
}

#[test]
fn test_audit_log_table_renderable() {
    let log = audit_logs::AuditLog {
        id: Some("log-1".to_string()),
        organization_id: None,
        tenant_id: None,
        operation: Some("UserLogin".to_string()),
        operation_time: Some("2025-01-01T10:00:00Z".to_string()),
        operator_user_id: None,
        operator_email: Some("user@test.com".to_string()),
        user_agent: None,
        ip: Some("1.2.3.4".to_string()),
        location: Some(audit_logs::AuditLocation {
            country: Some("Australia".to_string()),
            city: None,
            region: None,
        }),
    };
    let row = log.row();
    assert_eq!(row[0], "log-1");          // id
    assert_eq!(row[1], "UserLogin");      // operation
    assert_eq!(row[4], "user@test.com");  // email (index 4 in new headers)
    assert_eq!(row[5], "1.2.3.4");        // ip (index 5)
    assert_eq!(row[7], "Australia");      // country (index 7)
}

#[test]
fn test_audit_log_no_location() {
    let log = audit_logs::AuditLog {
        id: None, organization_id: None, tenant_id: None,
        operation: None, operation_time: None,
        operator_user_id: None, operator_email: None,
        ip: None, user_agent: None, location: None,
    };
    let row = log.row();
    assert_eq!(row[7], ""); // country should be empty (index 7 in new headers)
}

#[test]
fn test_compliance_framework_table_renderable() {
    let cf = compliance::ComplianceFramework {
        id: Some("CIS".to_string()),
        name: Some("CIS Benchmark".to_string()),
        version: Some("1.4".to_string()),
        posture: Some(82.1),
        passed_findings: Some(100),
        total_findings: Some(122),
        is_custom: Some(false),
        providers: Some(vec!["AWS".to_string(), "Azure".to_string()]),
        description: None,
        release_date: None,
        last_modified_date: None,
        framework_type: None,
        link: None,
    };
    let row = cf.row();
    assert_eq!(row[0], "CIS");
    assert_eq!(row[4], "82.1%");       // POSTURE % at index 4
    assert_eq!(row[7], "no");          // CUSTOM at index 7
    assert_eq!(row[8], "AWS, Azure");  // PROVIDERS at index 8
}

#[test]
fn test_well_architected_table_renderable() {
    let waf = well_architected::WellArchitectedFramework {
        id: Some("WAF".to_string()),
        name: Some("AWS WAF".to_string()),
        version: Some("2024".to_string()),
        posture: Some(75.0),
        passed_findings: Some(150),
        total_findings: Some(200),
        is_custom: None,
        providers: Some(vec!["AWS".to_string()]),
        description: None,
        release_date: None,
        last_modified_date: None,
        framework_type: None,
        link: None,
    };
    let row = waf.row();
    assert_eq!(row[0], "WAF");
    assert_eq!(row[4], "75.0%");  // POSTURE % at index 4
    assert_eq!(row[8], "AWS");    // PROVIDERS at index 8
}

#[test]
fn test_iac_scan_table_renderable() {
    let scan = iac::IacScan {
        id: Some("s-1".to_string()),
        artifact_name: Some("infra.zip".to_string()),
        status: Some("completed".to_string()),
        tenant_id: None,
        organization_id: None,
        created_at: Some("2025-01-01".to_string()),
        updated_at: None,
        summary: None,
        types: vec!["terraform".to_string()],
    };
    let row = scan.row();
    assert_eq!(row[0], "s-1");
    assert_eq!(row[1], "infra.zip");
    assert_eq!(row[3], "terraform");
}

#[test]
fn test_iac_finding_table_renderable() {
    let f = iac::IacFinding {
        id: Some("if-1".to_string()),
        scan_id: Some("s-1".to_string()),
        detection_id: Some("PLERION-K8S-108".to_string()),
        detection_title: Some("Ensure privileged is false".to_string()),
        finding_type: Some("terraform".to_string()),
        result: Some("FAILED".to_string()),
        severity_level: Some("HIGH".to_string()),
        file: Some("main.tf".to_string()),
        repository_path: None,
        line_range: Some(vec![10, 42]),
        resource: Some("aws_s3_bucket.data".to_string()),
        resource_tags: None,
        evaluated_keys: None,
        code_block: None,
        dashboard_url: None,
        tenant_id: None,
        organization_id: None,
        created_at: None,
        updated_at: None,
    };
    let row = f.row();
    assert_eq!(row[0], "if-1");
    assert_eq!(row[7], "main.tf");
    assert_eq!(row[8], "10-42");
}

// Deserialization tests

#[test]
fn test_finding_deserialization() {
    let json = r#"{
        "id": "prn:123",
        "detectionId": "PLERION-AWS-1",
        "status": "FAILED",
        "severityLevel": "CRITICAL"
    }"#;
    let f: findings::Finding = serde_json::from_str(json).unwrap();
    assert_eq!(f.detection_id.unwrap(), "PLERION-AWS-1");
    assert_eq!(f.status.unwrap(), "FAILED");
}

#[test]
fn test_finding_deserialization_extra_fields() {
    // Ensure unknown fields are ignored (forward compat)
    let json = r#"{
        "id": "prn:123",
        "unknownField": "should be ignored"
    }"#;
    let f: findings::Finding = serde_json::from_str(json).unwrap();
    assert_eq!(f.id.unwrap(), "prn:123");
}

#[test]
fn test_asset_deserialization() {
    let json = r#"{
        "id": "prn:assets:123",
        "name": "test",
        "type": "AWS::EC2::Instance",
        "riskScore": 9.36,
        "isPubliclyExposed": true
    }"#;
    let a: assets::Asset = serde_json::from_str(json).unwrap();
    assert_eq!(a.asset_type.unwrap(), "AWS::EC2::Instance");
    assert_eq!(a.is_publicly_exposed, Some(true));
}

#[test]
fn test_pagination_meta_deserialization() {
    let json = r#"{
        "cursor": "abc123",
        "perPage": 50,
        "total": 100,
        "hasNextPage": true,
        "hasPreviousPage": false
    }"#;
    let m: findings::PaginationMeta = serde_json::from_str(json).unwrap();
    assert_eq!(m.cursor.unwrap(), "abc123");
    assert_eq!(m.has_next_page, Some(true));
    assert_eq!(m.total, Some(100));
}

#[test]
fn test_page_pagination_meta_deserialization() {
    let json = r#"{ "page": 1, "perPage": 100, "total": 200, "hasNextPage": true, "hasPreviousPage": false }"#;
    let m: assets::PagePaginationMeta = serde_json::from_str(json).unwrap();
    assert_eq!(m.page, Some(1));
    assert_eq!(m.has_next_page, Some(true));
}
