pub mod detail_router_test;
pub mod main_router_test;
pub mod user_router_test;
pub mod manage_router_test;


mod tests {

use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};

pub struct GoogleKey {
  pub n : String,
  pub e : String,
  pub alg : String,
  pub _use : String,
  pub kid : String,
  pub kty : String
}
  #[test]
  fn sibal() {
      let google_key = GoogleKey {
        n: "yY0IYTajOWIdeweQB5ZMnvXquuSu2eDDOu1u2uw9_23YMe0nT72o-jBnHL4qG8UuEzYHeE6Smr8h-k75WqRC2aSOlaPFAoef9XYJ8CFFBgDPyWDWAqwmoOZeAIw3a_F6YmBA3CU0NcIYbrgFDVx-ZQmwj7VGUZJUno7MuafMK3lemcHx505j0TPmdrNfIJB3hVwFK7CvNxkRyE9lczm0HSbFnn8JXKxXimHUUqDa3Xh4v58gy2qsyUA8BWafvrrMJ5NdTOWU5gN2Ly7I4WcOT_ny2GsmQvUSdn9--NyZK3pQIPr158y6MFGxLZvYlCN4YqkHITial3WJ73l6HEIxBw".to_owned(),
        e: "AQAB".to_owned(),
        alg: "RS256".to_owned(),
        _use: "sig".to_owned(),
        kid: "b1a8259eb07660ef23781c85b7849bfa0a1c806c".to_owned(),
        kty: "RSA".to_owned()
    };

    let string = "eyJhbGciOiJSUzI1NiIsImtpZCI6ImIxYTgyNTllYjA3NjYwZWYyMzc4MWM4NWI3ODQ5YmZhMGExYzgwNmMiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJuYmYiOjE2NTI0MTgyOTEsImF1ZCI6IjQzNDIzNDA1OTM0MS0zZm9pcDM3Zjh1M3VjZ3NmNmR2ZHBnNWNuZ3BxaWVwNS5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsInN1YiI6IjExMjA4OTM3MDQ4MTc4NjA3Njg5NSIsImVtYWlsIjoicm5qc3Rrc2FvckBnbWFpbC5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiYXpwIjoiNDM0MjM0MDU5MzQxLTNmb2lwMzdmOHUzdWNnc2Y2ZHZkcGc1Y25ncHFpZXA1LmFwcHMuZ29vZ2xldXNlcmNvbnRlbnQuY29tIiwibmFtZSI6Iuq2jOyjvO2YhCIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS0vQU9oMTRHaEV2S3NqcGhHcURWTmZjOG12dEhJSU93and3b0FCV01wV2Jua0dTZz1zOTYtYyIsImdpdmVuX25hbWUiOiLso7ztmIQiLCJmYW1pbHlfbmFtZSI6Iuq2jCIsImlhdCI6MTY1MjQxODU5MSwiZXhwIjoxNjUyNDIyMTkxLCJqdGkiOiJmNGFmNmExOWJiNTM0MmUxN2I5ODY1MzEwZTY3YTY0NzZkOWUyMWUwIn0.uD6SopHtHWG0yIZ5lgymKBF6vteLgyeVL8e4ZDJm7fvioRSEukhV2306UJNeTouxiKy5soHxYkwGkXy3qqWnM0G8zs81z5QoGybrkuBY-jkeNRSaOuJTRDMuVap1KuQ3mXWoSfQlIOrph6NJCL3fxUTroiilcRp0W2rE5fzEW03b9kHuegpgzXOE44_6wop-0RbEg_SZKp-LRX9EIYBnAq-ZCylTQRUT8vWvWNdhcBkij7Rs3DEOlmrOYWhllFuXhtUy-_CMJKNKwN8mqdzp9zLIlfV-9DJnP0dabYM-QTCU8ioNfDv27jXaF61orQT1OOX9SSCxSy5jHarJ7Llqkg";

    let msg = jsonwebtoken::decode::<serde_json::Value>(string, &DecodingKey::from_rsa_components(&google_key.n, &google_key.e).unwrap(), &Validation::new(Algorithm::RS256)).unwrap();

    println!("{:#?}", msg.claims);

  }

}