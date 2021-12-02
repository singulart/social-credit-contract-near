use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{ext_contract, near_bindgen};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[ext_contract(ext_job_applications)]
pub trait JobApplications {
    fn job_applications_on_background_checked(&self, person: String, score: f32);
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct SocialCreditContract {}

#[near_bindgen]
impl SocialCreditContract {

    pub fn person_on_check_score(&self, applicant_id: String) {

        let person = applicant_id.clone();
        let person2 = applicant_id.clone();


        ext_job_applications::job_applications_on_background_checked(
            person,
            SocialCreditContract::get_score(person2), // social_credit
            &"job-service.isonar.testnet".to_string(),
            0,                             // attached yocto NEAR
            30000000000000,             // attached gas
        );

    }

    fn get_score(person: String) -> f32 {
        let ch = person.chars().next().unwrap();

        if ['a', 'i', 'e', 'o', 'u'].contains(&ch) {
            0.6
        } else {
            0.4
        }
    }
}