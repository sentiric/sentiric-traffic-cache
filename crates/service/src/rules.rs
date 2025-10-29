use sentiric_core::{Action, Rule, RuleCondition};
use tracing::debug;
use wildmatch::WildMatch;

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new(rules: Vec<Rule>) -> Self {
        Self { rules }
    }

    pub fn match_action(&self, uri: &str) -> Action {
        let parsed_uri = match url::Url::parse(uri) {
            Ok(u) => u,
            Err(_) => return Action::Allow, // URI parse edilemezse, varsayılan olarak izin ver
        };
        
        let domain = parsed_uri.domain().unwrap_or("");

        for rule in &self.rules {
            let matched = match &rule.condition {
                RuleCondition::Domain(d) => d == domain,
                RuleCondition::UrlPattern(p) => WildMatch::new(p).matches(uri),
            };

            if matched {
                debug!("Request to '{}' matched rule '{}'. Action: {:?}", uri, rule.name, rule.action);
                return rule.action.clone();
            }
        }

        // Hiçbir kural eşleşmezse varsayılan davranış
        Action::Allow
    }
}