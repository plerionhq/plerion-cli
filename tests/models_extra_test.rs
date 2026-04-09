use plerion::api::models::{
    alerts::Alert,
    asset_groups::AssetGroup,
    assets::Asset,
    audit_logs::AuditLog,
    compliance::ComplianceFramework,
    iac::{IacFinding, IacScan},
    integrations::Integration,
    vulnerabilities::{Vulnerability, VulnerabilityExemption},
    well_architected::WellArchitectedFramework,
};
use plerion::output::TableRenderable;

/// Test all models with all-None fields to ensure no panics.

#[test]
fn test_alert_all_nones() {
    let a = Alert {
        id: None,
        status: None,
        title: None,
        alert_type: None,
        risk_score: None,
        flagged: None,
        acknowledged: None,
        integration_id: None,
        workflow_id: None,
        created_at: None,
        updated_at: None,
        meta: None,
    };
    let row = a.row();
    assert_eq!(row.len(), Alert::headers().len());
}

#[test]
fn test_asset_group_all_nones() {
    let ag = AssetGroup {
        asset_group_id: None,
        name: None,
        status: None,
        total_assets: None,
        risk_score: None,
        tenant_id: None,
        organization_id: None,
        created_at: None,
        updated_at: None,
    };
    let row = ag.row();
    assert_eq!(row.len(), AssetGroup::headers().len());
}

#[test]
fn test_asset_all_nones() {
    let a = Asset {
        id: None,
        name: None,
        asset_type: None,
        provider: None,
        region: None,
        service: None,
        resource_type: None,
        resource_id: None,
        risk_score: None,
        is_publicly_exposed: None,
        is_vulnerable: None,
        has_kev: None,
        operational_state: None,
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
    assert_eq!(row.len(), Asset::headers().len());
}

#[test]
fn test_audit_log_all_nones() {
    let log = AuditLog {
        id: None,
        operation: None,
        operation_time: None,
        operator_user_id: None,
        operator_email: None,
        ip: None,
        user_agent: None,
        location: None,
    };
    let row = log.row();
    assert_eq!(row.len(), AuditLog::headers().len());
}

#[test]
fn test_compliance_framework_all_nones() {
    let cf = ComplianceFramework {
        id: None,
        name: None,
        version: None,
        posture: None,
        passed_findings: None,
        total_findings: None,
        is_custom: None,
        providers: None,
        description: None,
        release_date: None,
        link: None,
    };
    let row = cf.row();
    assert_eq!(row.len(), ComplianceFramework::headers().len());
}

#[test]
fn test_iac_scan_all_nones() {
    let s = IacScan {
        scan_id: None,
        artifact_name: None,
        status: None,
        tenant_id: None,
        organization_id: None,
        created_at: None,
    };
    let row = s.row();
    assert_eq!(row.len(), IacScan::headers().len());
}

#[test]
fn test_iac_finding_all_nones() {
    let f = IacFinding {
        id: None,
        severity_level: None,
        resource_type: None,
        message: None,
        file_path: None,
        line_number: None,
    };
    let row = f.row();
    assert_eq!(row.len(), IacFinding::headers().len());
}

#[test]
fn test_integration_all_nones() {
    let i = Integration {
        integration_id: None,
        name: None,
        provider: None,
        integration_type: None,
        status: None,
        risk_score: None,
        tenant_id: None,
        organization_id: None,
        created_at: None,
        updated_at: None,
        aws_account_id: None,
        azure_subscription_id: None,
        gcp_project_id: None,
    };
    let row = i.row();
    assert_eq!(row.len(), Integration::headers().len());
}

#[test]
fn test_vulnerability_all_nones() {
    let v = Vulnerability {
        vulnerability_id: None,
        title: None,
        severity_level: None,
        asset_id: None,
        asset_type: None,
        provider: None,
        has_kev: None,
        has_exploit: None,
        has_vendor_fix: None,
        first_observed_at: None,
        last_observed_at: None,
        is_exempted: None,
        primary_url: None,
        description: None,
    };
    let row = v.row();
    assert_eq!(row.len(), Vulnerability::headers().len());
}

#[test]
fn test_vulnerability_exemption_all_nones() {
    let e = VulnerabilityExemption {
        id: None,
        name: None,
        reason: None,
        created_at: None,
        updated_at: None,
    };
    let row = e.row();
    assert_eq!(row.len(), VulnerabilityExemption::headers().len());
}

#[test]
fn test_well_architected_all_nones() {
    let w = WellArchitectedFramework {
        id: None,
        name: None,
        version: None,
        posture: None,
        passed_findings: None,
        total_findings: None,
        is_custom: None,
        providers: None,
        description: None,
        release_date: None,
        link: None,
    };
    let row = w.row();
    assert_eq!(row.len(), WellArchitectedFramework::headers().len());
}

// --- Test deserialization from JSON with extra/missing fields ---

#[test]
fn test_alert_deserialize_extra_fields() {
    let json = serde_json::json!({
        "id": "prn:alerts:123",
        "status": "OPEN",
        "title": "High risk",
        "alertType": "FINDING",
        "riskScore": 9.5,
        "flagged": true,
        "acknowledged": false,
        "extra_field": "should be ignored"
    });
    let alert: Alert = serde_json::from_value(json).unwrap();
    assert_eq!(alert.status.as_deref(), Some("OPEN"));
    assert_eq!(alert.risk_score, Some(9.5));
    assert_eq!(alert.flagged, Some(true));
}

#[test]
fn test_vulnerability_short_title_not_truncated() {
    let v = Vulnerability {
        vulnerability_id: Some("CVE-2024-1".to_string()),
        title: Some("Short".to_string()),
        severity_level: Some("LOW".to_string()),
        asset_id: None,
        asset_type: None,
        provider: None,
        has_kev: None,
        has_exploit: None,
        has_vendor_fix: None,
        first_observed_at: None,
        last_observed_at: None,
        is_exempted: None,
        primary_url: None,
        description: None,
    };
    let row = v.row();
    assert_eq!(row[1], "Short");
}

#[test]
fn test_iac_scan_response_deserialize() {
    use plerion::api::models::iac::IacScanResponse;
    let json = serde_json::json!({
        "data": null,
        "meta": {
            "scanId": "scan-123",
            "artifactName": "test.zip",
            "tenantId": "t-1",
            "organizationId": "o-1"
        }
    });
    let resp: IacScanResponse = serde_json::from_value(json).unwrap();
    assert!(resp.data.is_none());
    assert_eq!(resp.meta.scan_id.as_deref(), Some("scan-123"));
}

#[test]
fn test_compliance_frameworks_response_deserialize() {
    use plerion::api::models::compliance::ComplianceFrameworksResponse;
    let json = serde_json::json!({
        "data": {
            "frameworks": [],
            "totalPosture": 95.5
        }
    });
    let resp: ComplianceFrameworksResponse = serde_json::from_value(json).unwrap();
    assert_eq!(resp.data.frameworks.len(), 0);
    assert_eq!(resp.data.total_posture, Some(95.5));
}

#[test]
fn test_asset_numeric_risk_score_deserialization() {
    let json = serde_json::json!({
        "riskScore": 7.5
    });
    let asset: Asset = serde_json::from_value(json).unwrap();
    assert_eq!(asset.risk_score, Some(serde_json::json!(7.5)));
}

#[test]
fn test_asset_long_id_truncated() {
    let long_id = format!("prn:some:very:long:prefix:{}", "a".repeat(50));
    let a = Asset {
        id: Some(long_id),
        name: None,
        asset_type: None,
        provider: None,
        region: None,
        service: None,
        resource_type: None,
        resource_id: None,
        risk_score: None,
        is_publicly_exposed: None,
        is_vulnerable: None,
        has_kev: None,
        operational_state: None,
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
    // Full ID is now shown (no truncation)
    assert!(row[0].starts_with("prn:"));
}

#[test]
fn test_asset_bool_fields_display() {
    let a = Asset {
        id: None,
        name: None,
        asset_type: None,
        provider: None,
        region: None,
        service: None,
        resource_type: None,
        resource_id: None,
        risk_score: None,
        is_publicly_exposed: Some(true),
        is_vulnerable: Some(false),
        has_kev: None,
        operational_state: None,
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
    assert_eq!(row[9], "yes");   // is_publicly_exposed
    assert_eq!(row[10], "no");   // is_vulnerable
}

#[test]
fn test_asset_string_risk_score_display() {
    let a = Asset {
        id: None,
        name: None,
        asset_type: None,
        provider: None,
        region: None,
        service: None,
        resource_type: None,
        resource_id: None,
        risk_score: Some(serde_json::json!("N/A")),
        is_publicly_exposed: None,
        is_vulnerable: None,
        has_kev: None,
        operational_state: None,
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
    assert_eq!(row[8], "N/A");
}
