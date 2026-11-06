use rmcp::{
    handler::server::wrapper::Parameters,
    schemars,
    tool,
    tool_router,
    transport::streamable_http_server::{
        StreamableHttpService,
        session::local::LocalSessionManager,
    },
    transport::StreamableHttpServerConfig,
};

use serde::Deserialize;

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CalcParams {
    a: f64,
    b: f64,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct ValueParams {
    value: f64,
}

#[derive(Clone)]
struct Calculator;

#[tool_router(server_handler)]
impl Calculator {
    // Addition
    #[tool(name = "add", description = "Add two numbers")]
    fn add(
        &self,
        Parameters(CalcParams { a, b }): Parameters<CalcParams>,
    ) -> String {
        (a + b).to_string()
    }

    // Subtraction
    #[tool(name = "subtract", description = "Subtract b from a")]
    fn subtract(
        &self,
        Parameters(CalcParams { a, b }): Parameters<CalcParams>,
    ) -> String {
        (a - b).to_string()
    }

    // Multiplication
    #[tool(name = "multiply", description = "Multiply two numbers")]
    fn multiply(
        &self,
        Parameters(CalcParams { a, b }): Parameters<CalcParams>,
    ) -> String {
        (a * b).to_string()
    }

    // Division
    #[tool(name = "divide", description = "Divide a by b")]
    fn divide(
        &self,
        Parameters(CalcParams { a, b }): Parameters<CalcParams>,
    ) -> String {
        if b == 0.0 {
            return "Error: Division by zero".to_string();
        }

        (a / b).to_string()
    }

    // Modulo
    #[tool(name = "modulo", description = "Calculate remainder of a divided by b")]
    fn modulo(
        &self,
        Parameters(CalcParams { a, b }): Parameters<CalcParams>,
    ) -> String {
        if b == 0.0 {
            return "Error: Modulo by zero".to_string();
        }

        (a % b).to_string()
    }

    // Power
    #[tool(name = "power", description = "Calculate a raised to the power b")]
    fn power(
        &self,
        Parameters(CalcParams { a, b }): Parameters<CalcParams>,
    ) -> String {
        a.powf(b).to_string()
    }

    // Square root
    #[tool(name = "sqrt", description = "Calculate square root of a number")]
    fn sqrt(
        &self,
        Parameters(ValueParams { value }): Parameters<ValueParams>,
    ) -> String {
        if value < 0.0 {
            return "Error: Cannot calculate square root of a negative number".to_string();
        }

        value.sqrt().to_string()
    }

    // Cube root
    #[tool(name = "cbrt", description = "Calculate cube root of a number")]
    fn cbrt(
        &self,
        Parameters(ValueParams { value }): Parameters<ValueParams>,
    ) -> String {
        value.cbrt().to_string()
    }

    // Absolute value
    #[tool(name = "abs", description = "Calculate absolute value")]
    fn abs(
        &self,
        Parameters(ValueParams { value }): Parameters<ValueParams>,
    ) -> String {
        value.abs().to_string()
    }

    // Factorial
    #[tool(name = "factorial", description = "Calculate factorial of a non-negative integer up to 170")]
    fn factorial(
        &self,
        Parameters(ValueParams { value }): Parameters<ValueParams>,
    ) -> String {
        if value < 0.0 || value.fract() != 0.0 || value > 170.0 {
            return "Error: Factorial requires an integer between 0 and 170".to_string();
        }

        let mut result = 1.0;

        for i in 1..=(value as u64) {
            result *= i as f64;
        }

        result.to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = StreamableHttpService::new(
        || Ok(Calculator),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default()
            .with_allowed_hosts(["dev.distridash.com"]),
    );

    let app = axum::Router::new().nest_service("/mcp", service);

    let listener =
        tokio::net::TcpListener::bind("127.0.0.1:8000").await?;

    println!("MCP Calculator running at http://127.0.0.1:8000/mcp");

    axum::serve(listener, app).await?;

    Ok(())
}
