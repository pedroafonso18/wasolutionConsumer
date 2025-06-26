# WaSolConsumer

## 📝 Descrição

**WaSolConsumer** é um consumidor RabbitMQ desenvolvido em Rust que processa requisições e webhooks do ecossistema WaSolCRM e wasolution. Ele atua como um componente intermediário que recebe mensagens de filas RabbitMQ e executa operações específicas como upsert de dados no banco PostgreSQL e envio de requisições HTTP para APIs externas.

O sistema é parte integrante do ecossistema WaSol, funcionando como um worker assíncrono que processa:
- Requisições de saída do CRM (enviadas para APIs externas)
- Webhooks recebidos das APIs de WhatsApp (Evolution e Wuzapi)
- Operações de banco de dados (chats, mensagens, clientes)

---

## 🚀 Funcionalidades

- **Consumo de Filas RabbitMQ**: Processa mensagens da fila `outgoing_requests`
- **Processamento de Dados**: Deserializa e processa diferentes tipos de mensagens
- **Operações de Banco**: Upsert de chats, mensagens e clientes no PostgreSQL
- **Requisições HTTP**: Envio de requisições para APIs externas
- **Logging Detalhado**: Sistema completo de logs para debugging
- **Reconexão Automática**: Reconecta automaticamente em caso de falhas
- **Graceful Shutdown**: Encerramento limpo com Ctrl+C

---

## 🛠️ Tecnologias Utilizadas

- **Rust** (linguagem principal)
- **Tokio** (runtime assíncrono)
- **Lapin** (cliente RabbitMQ)
- **PostgreSQL** (banco de dados via tokio-postgres)
- **Serde** (serialização/deserialização JSON)
- **Reqwest** (cliente HTTP)
- **Log** (sistema de logging)
- **Dotenvy** (variáveis de ambiente)

---

## 📋 Pré-requisitos

- Rust 1.70+ e Cargo
- PostgreSQL em execução
- RabbitMQ em execução
- Variáveis de ambiente configuradas (ver seção de configuração)

---

## ⚙️ Configuração

Crie um arquivo `.env` na raiz do projeto com o seguinte conteúdo:

```env
# URL do banco de dados PostgreSQL
DB_URL=postgres://usuario:senha@endereco:porta/nome_do_banco?sslmode=disable

# URL do RabbitMQ
RABBIT_URL=amqp://usuario:senha@endereco:porta/

# Configuração de logging (opcional)
RUST_LOG=debug
```

---

## 🏗️ Instalação e Execução

1. **Clone o repositório:**
   ```bash
   git clone <URL_DO_REPOSITORIO>
   cd WaSolConsumer
   ```

2. **Configure as variáveis de ambiente (.env)**

3. **Execute o consumidor:**
   ```bash
   cargo run
   ```

4. **Para produção, compile e execute:**
   ```bash
   cargo build --release
   ./target/release/WaSolConsumer
   ```

---

## 🔄 Tipos de Mensagens Processadas

### 1. **upsertChat**
Processa dados de chat para inserção/atualização no banco:

```json
{
  "id": 123,
  "situation": "active",
  "is_active": true,
  "agent_id": 456,
  "tabulation": "support",
  "customer_id": 789
}
```

### 2. **upsertCustomer**
Processa dados de cliente:

```json
{
  "id": 789,
  "name": "João Silva",
  "number": "5511999999999",
  "last_chat_id": "chat_123"
}
```

### 3. **upsertMessage**
Processa mensagens:

```json
{
  "id": 456,
  "from": "5511999999999",
  "to": "5511888888888",
  "delivered": true,
  "text": "Olá! Como posso ajudar?",
  "chat_id": 123
}
```

### 4. **sendRequest**
Envia requisições HTTP para APIs externas:

```json
{
  "action": "send_message",
  "method": "POST",
  "url": "https://api.whatsapp.com/send",
  "headers": {
    "Authorization": "Bearer token123",
    "Content-Type": "application/json"
  },
  "body": {
    "number": "5511999999999",
    "message": "Resposta automática"
  }
}
```

---

## 🗄️ Estrutura do Banco de Dados

O sistema espera as seguintes tabelas no PostgreSQL:

### Tabela `chats`
- `id` (INTEGER PRIMARY KEY)
- `situation` (TEXT)
- `is_active` (BOOLEAN)
- `agent_id` (INTEGER, NULLABLE)
- `tabulation` (TEXT, NULLABLE)
- `customer_id` (INTEGER)

### Tabela `customers`
- `id` (INTEGER PRIMARY KEY)
- `name` (TEXT)
- `number` (TEXT)
- `last_chat_id` (TEXT, NULLABLE)

### Tabela `messages`
- `id` (INTEGER PRIMARY KEY)
- `from` (TEXT)
- `to` (TEXT)
- `delivered` (BOOLEAN)
- `text` (TEXT)
- `chat_id` (INTEGER)

---

## 📦 Estrutura do Projeto

```
WaSolConsumer/
│
├── src/
│   ├── main.rs                 # Ponto de entrada da aplicação
│   ├── config/
│   │   ├── mod.rs
│   │   └── config.rs           # Carregamento de configurações
│   ├── rabbit/
│   │   ├── mod.rs
│   │   └── setup_rabbit.rs     # Configuração do RabbitMQ
│   ├── database/
│   │   ├── mod.rs
│   │   ├── connect.rs          # Conexão com PostgreSQL
│   │   └── insert.rs           # Operações de inserção
│   ├── parser/
│   │   ├── mod.rs
│   │   └── library.rs          # Estruturas de dados
│   ├── process/
│   │   ├── mod.rs
│   │   ├── outgoing.rs         # Processamento de saída
│   │   └── incoming.rs         # Processamento de entrada (futuro)
│   └── api/
│       ├── mod.rs
│       └── requests.rs         # Requisições HTTP
├── Cargo.toml                  # Dependências Rust
├── Cargo.lock
└── .env                        # Variáveis de ambiente
```

---

## 🔄 Fluxo de Processamento

1. **Conexão**: Conecta ao RabbitMQ e PostgreSQL
2. **Consumo**: Aguarda mensagens na fila `outgoing_requests`
3. **Deserialização**: Identifica o tipo de mensagem pelo conteúdo
4. **Processamento**: Executa a operação específica:
   - Upsert no banco de dados
   - Envio de requisição HTTP
5. **Confirmação**: Acknowledges a mensagem no RabbitMQ
6. **Logging**: Registra o resultado da operação

---

## 📊 Logs e Monitoramento

O sistema gera logs detalhados em diferentes níveis:

- **INFO**: Operações normais (conexões, processamento)
- **DEBUG**: Detalhes de mensagens recebidas
- **ERROR**: Erros de processamento e conexão
- **WARN**: Avisos sobre reconexões

Exemplo de logs:
```
[2024-01-15T10:30:00Z INFO  WaSolConsumer] Starting application
[2024-01-15T10:30:01Z INFO  WaSolConsumer] RabbitMQ connection established
[2024-01-15T10:30:02Z INFO  WaSolConsumer] Processing message of 245 bytes
[2024-01-15T10:30:02Z INFO  WaSolConsumer] Successfully processed outgoing request
```

---

## 🔒 Segurança

- **Validação de JSON**: Verifica estrutura das mensagens
- **Tratamento de Erros**: Captura e loga exceções
- **Reconexão Segura**: Reconecta automaticamente em falhas
- **Graceful Shutdown**: Encerra limpo com Ctrl+C

---

## 🔧 Troubleshooting

### Erro: "Couldn't deserialize data"
- Verifique se a mensagem JSON está no formato correto
- Confirme se contém um dos keywords esperados
- Verifique os logs para ver o conteúdo da mensagem

### Erro: "Failed to create RabbitMQ consumer"
- Verifique se o RabbitMQ está rodando
- Confirme a URL de conexão no `.env`
- Verifique credenciais de acesso

### Erro: "Couldn't connect to Database"
- Verifique se o PostgreSQL está rodando
- Confirme a URL de conexão no `.env`
- Verifique se as tabelas existem

---

## 🔮 Roadmap

- [X] Processamento de mensagens de saída
- [X] Integração com PostgreSQL
- [X] Sistema de logging completo
- [X] Reconexão automática
- [ ] Processamento de mensagens de entrada (webhooks)
- [ ] Suporte a múltiplas filas
- [ ] Métricas e monitoramento
- [ ] Interface de administração
- [ ] Testes automatizados
- [ ] Deploy com Docker

---

## 🤝 Integração com o Ecossistema

### WaSolCRM
- Recebe requisições de saída do CRM
- Processa operações de banco de dados
- Envia respostas para APIs externas

### wasolution
- Recebe webhooks das APIs de WhatsApp
- Processa notificações de mensagens
- Atualiza status de instâncias

---

## 📞 Suporte

Para suporte técnico ou dúvidas sobre o projeto, abra uma issue no repositório ou entre em contato com a equipe de desenvolvimento.

---

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.

---

## 🙏 Agradecimentos

- Equipe do WaSolCRM
- Comunidade Rust
- Projeto wasolution
- Todos os contribuidores
