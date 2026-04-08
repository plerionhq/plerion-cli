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
    assert_eq!(headers.len(), 5);
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
        detection_id: Some("PLERION-AWS-16".to_string()),
        status: Some("FAILED".to_string()),
        severity_level: Some("CRITICAL".to_string()),
        provider: Some("AWS".to_string()),
        resource_type: Some("AWS::S3::Bucket".to_string()),
        region: Some("us-east-1".to_string()),
        service: None,
        message: None,
        full_resource_name: None,
        asset_id: None,
        integration_id: None,
        first_observed_at: Some("2023-01-01".to_string()),
        last_observed_at: None,
        is_exempted: None,
        risk_score: None,
        calculated_severity: None,
        resource_url: None,
        tags: None,
    };
    let headers = findings::Finding::headers();
    assert_eq!(headers.len(), 7);
    let row = f.row();
    assert_eq!(row[0], "PLERION-AWS-16");
    let text_row = f.text_row();
    // text_row has no color codes
    assert_eq!(text_row[1], "FAILED");
    assert_eq!(text_row[2], "CRITICAL");
}

#[test]
fn test_finding_with_nones() {
    let f = findings::Finding {
        id: None, detection_id: None, status: None, severity_level: None,
        provider: None, resource_type: None, region: None, service: None,
        message: None, full_resource_name: None, asset_id: None,
        integration_id: None, first_observed_at: None, last_observed_at: None,
        is_exempted: None, risk_score: None, calculated_severity: None,
        resource_url: None, tags: None,
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
        name: Some("my-instance".to_string()),
        asset_type: None,
        provider: Some("AWS".to_string()),
        region: Some("us-east-1".to_string()),
        service: None,
        resource_type: Some("AWS::EC2::Instance".to_string()),
        resource_id: None,
        risk_score: Some(serde_json::json!(9.36)),
        is_publicly_exposed: Some(true),
        is_vulnerable: Some(false),
        has_kev: None,
        operational_state: Some("active".to_string()),
        first_observed_at: None,
        last_observed_at: None,
        integration_id: None,
        provider_account_id: None,
        number_of_critical_vulnerabilities: None,
        number_of_high_vulnerabilities: None,
        number_of_medium_vulnerabilities: None,
        number_of_low_vulnerabilities: None,
        resource_url: None,
    };
    let row = a.row();
    assert!(row[0].starts_with("...")); // truncated ID
    assert_eq!(row[1], "my-instance");
    assert_eq!(row[5], "9.36"); // risk score
    assert_eq!(row[6], "yes"); // publicly exposed
    assert_eq!(row[7], "no"); // vulnerable
}

#[test]
fn test_asset_string_risk_score() {
    let a = assets::Asset {
        id: Some("short-id".to_string()),
        name: None, asset_type: None, provider: None, region: None,
        service: None, resource_type: None, resource_id: None,
        risk_score: Some(serde_json::json!("7.5")),
        is_publicly_exposed: None, is_vulnerable: None, has_kev: None,
        operational_state: None, first_observed_at: None, last_observed_at: None,
        integration_id: None, provider_account_id: None,
        number_of_critical_vulnerabilities: None,
        number_of_high_vulnerabilities: None,
        number_of_medium_vulnerabilities: None,
        number_of_low_vulnerabilities: None,
        resource_url: None,
    };
    let row = a.row();
    assert_eq!(row[0], "short-id"); // no truncation for short IDs
    assert_eq!(row[5], "7.5"); // string risk score
}

#[test]
fn test_alert_table_renderable() {
    let a = alerts::Alert {
        id: Some("prn:alerts:abcdef01234567890123456789012345678901".to_string()),
        status: Some("OPEN".to_string()),
        title: Some("High risk asset".to_string()),
        alert_type: Some("ASSET".to_string()),
        risk_score: Some(9.5),
        flagged: Some(true),
        acknowledged: None,
        integration_id: None,
        workflow_id: None,
        created_at: Some("2025-01-01".to_string()),
        updated_at: None,
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
        description: None,
        score: Some(9.5),
        severity_level: Some("CRITICAL".to_string()),
        lifecycle_state: Some("OPEN".to_string()),
        primary_asset_id: None,
        region: Some("us-east-1".to_string()),
        integration_id: None,
        discovered_at: Some("2025-01-01".to_string()),
        meta: None,
    };
    let headers = risks::Risk::headers();
    assert_eq!(headers.len(), 7);
    let row = r.row();
    assert_eq!(row[0], "risk-1");
    assert_eq!(row[3], "9.50");
}

#[test]
fn test_vulnerability_table_renderable() {
    let v = vulnerabilities::Vulnerability {
        vulnerability_id: Some("CVE-2023-1234".to_string()),
        title: Some("A very long vulnerability title that exceeds fifty characters to test truncation".to_string()),
        severity_level: Some("HIGH".to_string()),
        asset_id: None,
        asset_type: Some("AWS::EC2::Instance".to_string()),
        provider: Some("AWS".to_string()),
        has_kev: Some(true),
        has_exploit: Some(false),
        has_vendor_fix: Some(true),
        first_observed_at: Some("2023-10-27".to_string()),
        last_observed_at: None,
        is_exempted: None,
        primary_url: None,
        description: None,
    };
    let row = v.row();
    assert_eq!(row[0], "CVE-2023-1234");
    assert!(row[1].len() <= 53); // truncated to 50 bytes + '…' (3 bytes UTF-8)
    assert_eq!(row[5], "yes"); // KEV
    assert_eq!(row[6], "no");  // exploit
    assert_eq!(row[7], "yes"); // fix
}

#[test]
fn test_vulnerability_exemption_table_renderable() {
    let e = vulnerabilities::VulnerabilityExemption {
        id: Some("ex-1".to_string()),
        name: Some("My exemption".to_string()),
        reason: Some("ACCEPTED_RISK".to_string()),
        created_at: Some("2025-01-01".to_string()),
        updated_at: None,
    };
    let row = e.row();
    assert_eq!(row[0], "ex-1");
    assert_eq!(row[1], "My exemption");
    assert_eq!(row[2], "ACCEPTED_RISK");
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
        created_at: None,
        updated_at: None,
        aws_account_id: Some("123456789012".to_string()),
        azure_subscription_id: None,
        gcp_project_id: None,
    };
    let row = i.row();
    assert_eq!(row[0], "int-1");
    assert_eq!(row[1], "AWS Prod");
    assert_eq!(row[6], "123456789012"); // account
}

#[test]
fn test_integration_azure_account() {
    let i = integrations::Integration {
        integration_id: None, name: None, provider: Some("Azure".to_string()),
        integration_type: None, status: None, risk_score: None,
        tenant_id: None, organization_id: None, created_at: None, updated_at: None,
        aws_account_id: None,
        azure_subscription_id: Some("sub-123".to_string()),
        gcp_project_id: None,
    };
    let row = i.row();
    assert_eq!(row[6], "sub-123");
}

#[test]
fn test_integration_gcp_project() {
    let i = integrations::Integration {
        integration_id: None, name: None, provider: Some("GCP".to_string()),
        integration_type: None, status: None, risk_score: None,
        tenant_id: None, organization_id: None, created_at: None, updated_at: None,
        aws_account_id: None, azure_subscription_id: None,
        gcp_project_id: Some("my-project".to_string()),
    };
    let row = i.row();
    assert_eq!(row[6], "my-project");
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
        operation: Some("UserLogin".to_string()),
        operation_time: Some("2025-01-01T10:00:00Z".to_string()),
        operator_user_id: None,
        operator_email: Some("user@test.com".to_string()),
        ip: Some("1.2.3.4".to_string()),
        user_agent: None,
        location: Some(audit_logs::AuditLocation {
            country: Some("Australia".to_string()),
            city: None,
            region: None,
        }),
    };
    let row = log.row();
    assert_eq!(row[0], "UserLogin");
    assert_eq!(row[1], "user@test.com");
    assert_eq!(row[2], "1.2.3.4");
    assert_eq!(row[3], "Australia");
}

#[test]
fn test_audit_log_no_location() {
    let log = audit_logs::AuditLog {
        id: None, operation: None, operation_time: None,
        operator_user_id: None, operator_email: None,
        ip: None, user_agent: None, location: None,
    };
    let row = log.row();
    assert_eq!(row[3], ""); // country should be empty
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
        link: None,
    };
    let row = cf.row();
    assert_eq!(row[0], "CIS");
    assert_eq!(row[3], "82.1%");
    assert_eq!(row[6], "no");
    assert_eq!(row[7], "AWS, Azure");
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
        link: None,
    };
    let row = waf.row();
    assert_eq!(row[0], "WAF");
    assert_eq!(row[3], "75.0%");
    assert_eq!(row[6], "AWS");
}

#[test]
fn test_iac_scan_table_renderable() {
    let scan = iac::IacScan {
        scan_id: Some("s-1".to_string()),
        artifact_name: Some("infra.zip".to_string()),
        status: Some("completed".to_string()),
        tenant_id: None,
        organization_id: None,
        created_at: Some("2025-01-01".to_string()),
    };
    let row = scan.row();
    assert_eq!(row[0], "s-1");
    assert_eq!(row[1], "infra.zip");
}

#[test]
fn test_iac_finding_table_renderable() {
    let f = iac::IacFinding {
        id: Some("if-1".to_string()),
        severity_level: Some("HIGH".to_string()),
        resource_type: Some("AWS::S3::Bucket".to_string()),
        message: Some("Public bucket".to_string()),
        file_path: Some("main.tf".to_string()),
        line_number: Some(42),
    };
    let row = f.row();
    assert_eq!(row[0], "if-1");
    assert_eq!(row[3], "main.tf");
    assert_eq!(row[4], "42");
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
