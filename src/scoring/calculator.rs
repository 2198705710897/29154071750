pub struct TokenScorer;

impl TokenScorer {
    /// Calculate token score based on migration status, migration speed, and ATH
    ///
    /// Scoring System:
    /// - Score 0: Migrated + ATH >= $100,000
    /// - Score 1: Migrated + fast migration (<=2.5h)
    /// - Score 2: Migrated + slow migration (>2.5h)
    /// - Score 3: Not migrated + ATH >= $30,000
    /// - Score 4: Not migrated + ATH $20k-$30k
    /// - Score 5: Not migrated + ATH $10k-$20k
    /// - Score 6: Not migrated + ATH < $10k
    pub fn calculate_score(
        ath_market_cap: Option<&str>,
        is_migrated: bool,
        migrated_at: Option<i64>,
        created_at: Option<i64>,
    ) -> Option<i64> {
        let ath_value = ath_market_cap.and_then(|s| s.parse::<f64>().ok());

        // Score 0: Migrated + ATH >= $100,000 (highest priority)
        if is_migrated && ath_value.map_or(false, |ath| ath >= 100_000.0) {
            return Some(0);
        }

        // Score 1 & 2: Migrated (check migration speed)
        if is_migrated {
            if let (Some(migrated), Some(created)) = (migrated_at, created_at) {
                let duration_hours = (migrated - created) as f64 / 3600.0;
                if duration_hours <= 2.5 {
                    return Some(1);  // Fast migration
                } else {
                    return Some(2);  // Slow migration
                }
            }
            // Migrated but can't calculate speed -> assume slow
            return Some(2);
        }

        // Scores 3-6: Not migrated (score by ATH only)
        if let Some(ath) = ath_value {
            if ath >= 30_000.0 {
                return Some(3);
            } else if ath >= 20_000.0 {
                return Some(4);
            } else if ath >= 10_000.0 {
                return Some(5);
            }
        }

        // Score 6: ATH < $10k or unknown
        Some(6)
    }
}
