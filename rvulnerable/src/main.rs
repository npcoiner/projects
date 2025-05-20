use rand::Rng;

fn main() {
    let user: &str = "Nick";
    let mut logged_out: bool = false;
    let mut rng = rand::rng();
    let number: u8 = rng.random_range(1..=100);

    if number > 50 {
        logged_out = true;
    }

    if auth_user(user, logged_out){
        println!("Welcome authorized user!");
    }
    else {
        println!("You are not authed");
    }

}

fn auth_user(user: &str, logged_out: bool) -> bool {
    if logged_out{
        log_user_in(user)// Oops
    }
    else{
        api_check_user_auth(user)
    }
}

fn api_check_user_auth(user: &str) -> bool{
    if user == "Nicholas" {
        true   
    }
    else{
        false
    }
}

fn log_user_in(_user: &str) -> bool{
    //log user in
    true // confirm logged in
}
