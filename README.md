# Projeto ESP32 em Rust

Projeto configurado para desenvolvimento em Rust para a placa **ESP32** (Xtensa dual-core) utilizando a abordagem `std` com `esp-idf-svc`.

## Pré-requisitos

Para compilar para ESP32 (Xtensa), instale o `espup` e as ferramentas do ecossistema ESP-RS:

1. **Instalar `cargo-espflash` e `ldproxy`:**
   ```bash
   cargo install cargo-espflash ldproxy
   ```

2. **Instalar o toolchain Xtensa com `espup`:**
   ```bash
   cargo install espup
   espup install
   ```

3. **Carregar as variáveis de ambiente geradas pelo `espup`:**
   ```bash
   . $HOME/export-esp.sh
   ```

## Compilação e Execução

- **Compilar o projeto:**
  ```bash
  cargo build
  ```

- **Gravar na placa e monitorar os logs:**
  ```bash
  cargo run
  # ou
  cargo espflash flash --monitor
  ```

## Estrutura do Projeto

- `Cargo.toml`: Dependências do Rust (`esp-idf-svc`, `log`, `embuild`).
- `.cargo/config.toml`: Configuração do target `xtensa-esp32-espidf`, linker `ldproxy` e runner `espflash`.
- `rust-toolchain.toml`: Define o canal de toolchain `esp`.
- `build.rs`: Script de build integrando com `embuild`.
- `sdkconfig.defaults`: Configurações padrão do ESP-IDF (ex: tamanho de stack ajustado para Rust).
- `src/main.rs`: Ponto de entrada da aplicação.
