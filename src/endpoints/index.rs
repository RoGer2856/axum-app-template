use axum::response::Html;
use axum_helpers::auth::LoginInfoExtractor;

use crate::model::login_info::LoginInfo;

use super::{get_index_html_head, get_index_html_tail};

pub async fn index(login_info: Option<LoginInfoExtractor<LoginInfo>>) -> Html<String> {
    let header = if login_info.is_some() {
        r#"
            <link rel="stylesheet" href="public/main.css">

            <script>
                async function logout(event) {
                    event.preventDefault();

                    await fetch("/api/logout", {
                        method: "POST",
                    });

                    location.reload();
                }
            </script>

            <form onsubmit="logout(event)">
                <button class="button">Logout</button>
            </form>
        "#
    } else {
        r#"
            <div><a href="/login">Login</a></div>
        "#
    };

    Html(format!(
        r#"
            {}
                {header}
                <h1>Endpoints</h1>
                <ul>
                    <li><b>get /</b>: returns this page</li>
                    <li><b>get /login</b>: returns a page where a user can log in</li>

                    <li><b>post /api/login</b>: logs a user in</li>
                    <li><b>post /api/logout</b>: logs a user out</li>
                </ul>
            {}
        "#,
        get_index_html_head(),
        get_index_html_tail()
    ))
}
