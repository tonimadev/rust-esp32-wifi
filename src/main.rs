use std::thread::sleep;
use std::time::Duration;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::log::EspLogger;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use log::{error, info, warn};

const WIFI_SSID: &str = "SUA_REDE_WIFI";
const WIFI_PASS: &str = "SUA_SENHA_WIFI";

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    EspLogger::initialize_default();

    info!("Iniciando aplicação ESP32...");

    let peripherals = Peripherals::take().expect("Falha ao obter periféricos");
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;

    info!("Inicializando driver Wi-Fi...");
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
    )?;

    info!("Configurando Wi-Fi no modo Station (Client)...");
    let auth_method = if WIFI_PASS.is_empty() {
        AuthMethod::None
    } else {
        AuthMethod::WPA2Personal
    };

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: WIFI_SSID.try_into().unwrap_or_default(),
        password: WIFI_PASS.try_into().unwrap_or_default(),
        auth_method,
        ..Default::default()
    }))?;

    info!("Iniciando Wi-Fi...");
    wifi.start()?;

    info!("Escaneando pontos de acesso disponíveis nas proximidades...");
    match wifi.scan_n::<10>() {
        Ok((ap_list, count)) => {
            info!("Encontradas {} redes Wi-Fi (mostrando {}):", count, ap_list.len());
            for ap in ap_list {
                info!("  - SSID: '{}' | Sinal (RSSI): {} dBm | Canal: {}", ap.ssid, ap.signal_strength, ap.channel);
            }
        }
        Err(e) => {
            warn!("Não foi possível escanear redes: {:?}", e);
        }
    }

    if WIFI_SSID != "SUA_REDE_WIFI" {
        info!("Tentando conectar ao SSID '{}'...", WIFI_SSID);
        match wifi.connect() {
            Ok(_) => {
                info!("Conectado ao AP com sucesso! Aguardando atribuição de IP via DHCP...");
                match wifi.wait_netif_up() {
                    Ok(_) => {
                        let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
                        info!("Rede configurada com sucesso!");
                        info!("Endereço IP: {}", ip_info.ip);
                        info!("Máscara de sub-rede: {}", ip_info.subnet.mask);
                        info!("Gateway: {}", ip_info.subnet.gateway);
                    }
                    Err(e) => {
                        error!("Falha ao aguardar obtenção de IP: {:?}", e);
                    }
                }
            }
            Err(e) => {
                error!("Falha ao conectar no Wi-Fi: {:?}", e);
            }
        }
    } else {
        warn!("SSID padrão detectado ('SUA_REDE_WIFI').");
        warn!("Para conectar a uma rede específica, altere as constantes WIFI_SSID e WIFI_PASS em src/main.rs.");
    }

    info!("ESP32 em execução. Entrando em loop de monitoramento...");
    loop {
        if wifi.is_connected().unwrap_or(false) {
            info!("Status Wi-Fi: CONECTADO | IP: {:?}", wifi.wifi().sta_netif().get_ip_info().map(|i| i.ip));
        } else {
            info!("Status Wi-Fi: DESCONECTADO / AGUARDANDO");
        }
        sleep(Duration::from_secs(10));
    }
}
