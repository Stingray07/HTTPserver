pub fn handle_get_user() -> Vec<u8> {
    html_response(Status::Ok, "HOME", "HOME")
}