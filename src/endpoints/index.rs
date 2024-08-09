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

                    <li><b>get /api/seen-users</b>: lists the users seen since the server started</li>
                    <li><b>get /api/seen-users/:index</b>: shows user at the given position</li>

                    <li><b>get /api/create-uuid-v4</b>: generates and returns a uuid value (v4)</li>
                    <li><b>get /api/echo/:this/and/:that</b>: returns this and that in a json object</li>
                    <li><b>get /api/echo-path</b>: returns the path of the request in a json object</li>
                    <li><b>get /api/echo-query-params</b>: returns all query params in a list</li>
                    <li><b>get /api/echo-parsed-query-params?uuid=<uuid>&list=<item0>&list=<item1></b>: parses the query params and returns them in a list in a json object</li>
                    <li><b>get /api/echo-uuid-in-path/:uuid</b>: returns the uuid in the path</li>
                </ul>
            {}
        "#,
        get_index_html_head(),
        get_index_html_tail()
    ))
}
