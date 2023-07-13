use anyhow::Result;
use certify::{generate_ca, generate_cert, load_ca, CertType, CA};
use tokio::fs;

struct CertPem {
    cert_type: CertType,
    cert: String,
    key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 创建 CA 证书
    let pem = create_ca()?;
    gen_files(&pem).await?;
    let ca = load_ca(&pem.cert, &pem.key)?;
    // 创建服务器cert证书
    let pem = create_cert(&ca, &["kvserver.acme.inc"], "Acme KV server", false)?;
    gen_files(&pem).await?;
    // 创建客户端cert证书
    let pem = create_cert(&ca, &[], "awesome-device-id", true)?;
    gen_files(&pem).await?;
    Ok(())
}

/// 生成 CA 证书
fn create_ca() -> Result<CertPem> {
    let (cert, key) = generate_ca(
        &["acme.inc"],
        "CN",
        "Acme Inc.",
        "Acme CA",
        None,
        Some(10 * 365),
    )?;
    Ok(CertPem {
        cert_type: CertType::CA,
        cert,
        key,
    })
}

/// 生成 Cert 证书（客户端，服务端）
fn create_cert(ca: &CA, domains: &[&str], cn: &str, is_client: bool) -> Result<CertPem> {
    let (days, cert_type) = if is_client {
        (Some(365), CertType::Client)
    } else {
        (Some(5 * 365), CertType::Server)
    };
    let (cert, key) = generate_cert(ca, domains, "CN", "Acme Inc.", cn, None, is_client, days)?;

    Ok(CertPem {
        cert_type,
        cert,
        key,
    })
}

/// 证书保存到文件
async fn gen_files(pem: &CertPem) -> Result<()> {
    let name = match pem.cert_type {
        CertType::Client => "client",
        CertType::Server => "server",
        CertType::CA => "ca",
    };
    fs::write(format!("fixtures/{}.cert", name), pem.cert.as_bytes()).await?;
    fs::write(format!("fixtures/{}.key", name), pem.key.as_bytes()).await?;
    Ok(())
}
