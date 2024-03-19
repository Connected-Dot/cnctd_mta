#[cfg(test)]
mod tests {
    use cnctd_mta::MTA;

    #[tokio::test]
    async fn test_commands() {
        let vehicles = MTA::get_all_lines().await.unwrap();
        println!("vehicles: {:?}", vehicles);
    }
}
