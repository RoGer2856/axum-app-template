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

                    <li><b><a href="/api/seen-users">get /api/seen-users</a></b>: lists the users seen since the server started</li>
                    <li><b><a href="/api/seen-users/0">get /api/seen-users/:index</a></b>: shows user at the given position</li>

                    <li><b><a href="/api/create-uuid-v4">get /api/create-uuid-v4</a></b>: generates and returns a uuid value (v4)</li>
                    <li><b><a href="/api/echo/foo/and/bar">get /api/echo/:this/and/:that</a></b>: returns this and that in a json object</li>
                    <li><b><a href="/api/echo-path">get /api/echo-path</a></b>: returns the path of the request in a json object</li>
                    <li><b><a href="/api/echo-query-params?key0=value0&key1=value1&listkey=a&listkey=b">get /api/echo-query-params</a></b>: returns all query params in a list</li>
                    <li><b><a href="/api/echo-parsed-query-params?uuid=88292365-1919-4e00-b406-6988740f395c&list=value0&list=value1">get /api/echo-parsed-query-params?uuid=:uuid&list=:item0&list=:item1</a></b>: parses the query params and returns them in a list in a json object</li>
                    <li><b><a href="/api/echo-uuid-in-path/88292365-1919-4e00-b406-6988740f395c">get /api/echo-uuid-in-path/:uuid</a></b>: returns the uuid in the path</li>
                </ul>
            {}
        "#,
        get_index_html_head(),
        get_index_html_tail()
    ))
}
