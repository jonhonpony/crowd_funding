#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::test::request;


    #[test]
    fn test_stellar_public_key_from_secret() {
        let public_key = stellar_public_key_from_secret("SDA3ACRYZFUKLPDPN7XXDKUWLFQWMLUC72QS4XDHEESAL546BWHXF5JC").unwrap();
        assert_eq!(public_key, "GDTCHOOGATSGQT7CJ72UIEJT7SNOUUES4XTUWCSDXHVQSKUSZ4XQFPVJ");
    }

    #[test]
    fn test_get_account_info() {
        let result = get_account_info("GDTCHOOGATSGQT7CJ72UIEJT7SNOUUES4XTUWCSDXHVQSKUSZ4XQFPVJ");
        assert!(result.is_ok());
        let account_info = result.unwrap();
        assert!(account_info.sequence.parse::<u64>().is_ok());
    }

    #[test]
    fn test_create_transaction() {
        let result = create_transaction("SDA3ACRYZFUKLPDPN7XXDKUWLFQWMLUC72QS4XDHEESAL546BWHXF5JC", "GDTCHOOGATSGQT7CJ72UIEJT7SNOUUES4XTUWCSDXHVQSKUSZ4XQFPVJ", "100");
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("transaction"));
        
    }
}