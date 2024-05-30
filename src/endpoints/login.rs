use axum::response::Html;
use axum_helpers::auth::LoginInfoExtractor;

use crate::model::login_info::LoginInfo;

pub async fn login(login_info: Option<LoginInfoExtractor<LoginInfo>>) -> Html<String> {
    let body_content = if login_info.is_some() {
        r#"
            You are already logged in!
        "#
    } else {
        r#"
            <link rel="stylesheet" href="public/main.css">

            <script>
                async function login(event) {
                    event.preventDefault();

                    let loginname = document.getElementById("loginname").value;
                    let password = document.getElementById("password").value;

                    await fetch("/api/login", {
                        method: "POST",
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({
                            loginname,
                            password,
                        }),
                    });

                    location = "/";
                }
            </script>

            <h1>Login</h1>

            <form onsubmit="login(event)">
                <label for="loginname">Loginname</label>
                <input type="username" id="loginname" />

                <label for="password">Password</label>
                <input type="password" id="password" />

                <button class="button">Login</button>
            </form>
        "#
    };

    Html(format!(
        r#"
            <html>
                <body>
                    {body_content}
                </body>
            </html>
        "#
    ))
}
