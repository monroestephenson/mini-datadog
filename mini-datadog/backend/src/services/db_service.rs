use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use anyhow::Result;
use crate::handlers::metrics_handler::{Metric, CustomMetric};

pub struct DbService {
    pool: Pool<Postgres>,
}

impl DbService {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn store_metric(&self, metric: &Metric) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO system_metrics (timestamp, cpu, memory, latency, request_count)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            chrono::Utc::now(),
            metric.cpu,
            metric.memory,
            metric.latency,
            metric.request_count,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn store_custom_metric(&self, metric: &CustomMetric) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO custom_metrics (name, value, labels, timestamp)
            VALUES ($1, $2, $3, $4)
            "#,
            metric.name,
            metric.value,
            serde_json::to_value(&metric.labels)?,
            metric.timestamp,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_recent_metrics(&self, limit: i32) -> Result<Vec<Metric>> {
        let rows = sqlx::query!(
            r#"
            SELECT timestamp, cpu, memory, latency, request_count
            FROM system_metrics
            ORDER BY timestamp DESC
            LIMIT $1
            "#,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Metric {
                time: row.timestamp.format("%H:%M:%S").to_string(),
                cpu: row.cpu,
                memory: row.memory,
                latency: row.latency,
                request_count: row.request_count,
            })
            .collect())
    }

    pub async fn cleanup_old_metrics(&self, days: i32) -> Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM system_metrics
            WHERE timestamp < NOW() - INTERVAL '1 day' * $1
            "#,
            days as f64
        )
        .execute(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM custom_metrics
            WHERE timestamp < NOW() - INTERVAL '1 day' * $1
            "#,
            days as f64
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
} 