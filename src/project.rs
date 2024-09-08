use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub owner: String,
    pub target_amount: u64,
    pub current_amount: u64,
    pub is_funded: bool,
    pub backers: Vec<(String, u64)>,
}

impl Project {
    pub fn new(name: &str, owner: &str, target_amount: u64) -> Self {
        Project {
            name: name.to_string(),
            owner: owner.to_string(),
            target_amount,
            current_amount: 0,
            is_funded: false,
            backers: vec![],
        }
    }

    pub fn transfer_funds(&mut self) {
        println!("Proje için fonlar aktarılıyor: {}", self.name);

        // Proje sahibine fonların aktarılmasını simüle edin
        let total_funds = self.current_amount;
        let owner_address = &self.owner;

        // Burada gerçek transfer mantığını uygulayacaksınız
        // Örneğin, bir blok zinciri SDK'sı veya ödeme API'si kullanarak
        println!(
            "{} birim para birimi proje sahibine aktarılıyor: {}",
            total_funds, owner_address
        );

        // Destekçilerden proje sahibine fonların aktarılmasını simüle edin
        for (backer, amount) in &self.backers {
            println!(
                "{} birim para birimi destekçiden aktarılıyor: {} proje sahibine: {}",
                amount, backer, owner_address
            );
        }

        // Projeyi fonlanmış olarak işaretleyin
        if total_funds >= self.target_amount {
            self.is_funded = true;
            println!("Proje {} tamamen fonlandı!", self.name);
        } else {
            println!("Proje {} henüz tamamen fonlanmadı.", self.name);
        }
    }
}


