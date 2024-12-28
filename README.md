# Encryo Service

Encryo is a lightweight and efficient encryption and decryption microservice built with Rust. It uses the AES-GCM encryption standard for secure and high-performance data handling. The service exposes two simple endpoints for encryption and decryption of data, designed to be easily deployable in Docker or Kubernetes environments.

## Features

- **High Performance**: Built with Rust for low-latency and efficient memory usage.
- **Secure Encryption**: Uses AES-GCM with strong security guarantees.
- **Simple API**: RESTful endpoints for easy integration.
- **Scalable**: Docker-ready and Kubernetes-compatible.

## Requirements

- Rust 1.72 or higher
- Docker (optional, for containerized deployment)
- Cargo (Rust's package manager)

## API Endpoints

### 1. Encrypt Data
**POST /encrypt**

Encrypts the given plaintext data.

**Request Body:**
```json
{
    "data": "example@gmail.com"
}
```

**Response:**
```json
{
    "encrypted": "Base64-encrypted-string"
}
```

### 2. Decrypt Data
**POST /decrypt**

Decrypts the provided encrypted data.

**Request Body:**
```json
{
    "encrypted": "Base64-encrypted-string"
}
```

**Response:**
```json
{
    "data": "example@gmail.com"
}
```

## Installation

### 1. Run Locally

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/encryo-service.git
   cd encryo-service
   ```

2. Build and run the service:
   ```bash
   cargo run --release
   ```

3. The service will be available at `http://localhost:3030`.

### 2. Run with Docker

1. Build the Docker image:
   ```bash
   docker build -t encryo-service .
   ```

2. Run the container:
   ```bash
   docker run -p 3030:3030 encryo-service
   ```

## Environment Variables

| Variable          | Description                              | Default |
|-------------------|------------------------------------------|---------|
| `ENCRYPTION_KEY`  | Encryption key for AES-GCM (32 bytes)   | Randomly generated |

## Deployment

### Deploy with Kubernetes

1. Create a `deployment.yaml` file:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: encryo-service
spec:
  replicas: 3
  selector:
    matchLabels:
      app: encryo-service
  template:
    metadata:
      labels:
        app: encryo-service
    spec:
      containers:
      - name: encryo-service
        image: your-dockerhub-username/encryo-service:latest
        ports:
        - containerPort: 3030
```

2. Apply the deployment:
   ```bash
   kubectl apply -f deployment.yaml
   ```

## License

This project is licensed under the [MIT License](LICENSE).

## Contributing

Contributions are welcome! Please submit a pull request or open an issue if you find a bug or have a feature request.
