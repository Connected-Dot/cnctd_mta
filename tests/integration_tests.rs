#[cfg(test)]
mod tests {
    use cnctd_mta::MTA;

    #[tokio::test]
    async fn test_commands() {
        let vehicles = MTA::get_line("g").await.unwrap();
        println!("vehicles: {:?}", vehicles);
    }
}
