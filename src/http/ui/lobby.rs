//! Port of `assets/templates/client.html` (now deleted). Structure/ids are
//! preserved exactly, since `assets/static/scripts/{client,chat}.js` (kept
//! untouched - see `crate::http::assets` module docs) select on them.

use leptos::prelude::*;

use super::Banner;

#[component]
pub fn LobbyPage(
    socket_server: String,
    game_version: String,
    allow_password_reset: bool,
    admin_password_reset: bool,
    reset_token: Option<String>,
    reset_token_error: Option<String>,
) -> impl IntoView {
    view! {
        <html>
            <head>
                <title>"WebTiles - Dungeon Crawl Stone Soup"</title>
                <link rel="icon" href="/static/stone_soup_icon-32x32.png" r#type="image/png"/>
                <script r#type="text/javascript">
                    {format!(
                        "var socket_server = \"{socket_server}\";\nvar game_version = \"{game_version}\";",
                    )}
                </script>
                <script src="/static/scripts/contrib/require.js" data-main="/static/scripts/app"></script>
                <link rel="stylesheet" r#type="text/css" href="/static/style.css"/>
                <link rel="stylesheet" r#type="text/css" href="/static/fonts/fonts.css"/>
                <meta name="apple-mobile-web-app-capable" content="yes"/>
                <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent"/>
                <meta name="viewport" content="width=860, initial-scale=1.0"/>
            </head>
            <body>
                <noscript>"Please enable javascript!"</noscript>

                <div id="lobby" style="display: none;">
                    <LoginBox allow_password_reset=allow_password_reset/>
                    <RegisterDialog/>
                    <ChangePasswordDialog/>
                    <ChangeEmailDialog/>
                    <div id="floating_ok_message" class="floating_dialog" style="display: none;">
                        <span id="ok_message_content"></span>
                        <input class="button" r#type="submit" name="submit" value="OK"/>
                    </div>

                    {allow_password_reset.then(|| view! { <ForgotPasswordDialogs/> })}
                    {reset_token.map(|token| view! { <ResetPasswordDialog token=token error=reset_token_error/> })}

                    <RcEditDialog/>
                    <ExitGameDialog/>

                    <div id="banner"><Banner username=None/></div>

                    <div id="account_restricted" style="display:none;">
                        "This account is being held for administrator approval. Until approved,
                        community features and some game modes may be restricted, and games
                        will not be visible to other players."
                    </div>
                    <AdminPanel admin_password_reset=admin_password_reset/>

                    <div id="play_now"></div>

                    <br/>

                    <LobbyBody/>

                    <div id="footer"></div>
                </div>

                <div id="game">
                    <div id="crt" style="display: none;"></div>
                </div>

                <div id="loader">
                    <div id="loader_center">
                        <span id="loader_text">"Loading..."</span>
                        <br/>
                        <LoaderImages/>
                    </div>

                    <div id="stale_processes_message" class="floating_dialog"
                         style="display: none;" tabindex="1000">
                        "There are some stale "<span class="game_name"></span>" processes.
                        They'll be stopped in "<span class="recover_timeout"></span>" seconds.
                        Press a key now if you don't want this to happen!"
                    </div>

                    <div id="force_terminate" class="floating_dialog"
                         style="display: none;" tabindex="1001">
                        "Couldn't stop one of your stale "<span class="game_name"></span>" processes
                        gracefully. Force its termination? [yn]"
                        <br/>
                        <input class="button" r#type="button" name="force_terminate_no"
                               id="force_terminate_no" value="No" style="float: right;"/>
                        <input class="button" r#type="button" name="force_terminate_yes"
                               id="force_terminate_yes" value="Yes" style="float: right;"/>
                    </div>
                </div>

                <div id="overlay"></div>

                <div id="chat_hidden" style="display: none;"><span><a href="javascript:">"+"</a></span></div>

                <ChatWidget/>

                <div id="prompt" style="display: none;">
                    <div>
                        <div id="prompt_title"></div>
                        <div class="login_placeholder"></div>
                        <div id="prompt_footer"></div>
                    </div>
                </div>
            </body>
        </html>
    }
}

#[component]
fn LoginBox(allow_password_reset: bool) -> impl IntoView {
    view! {
        <div class="login_placeholder">
            <div id="login">
                <span id="login_message"></span>
                <form action="#" id="login_form">
                    <label r#for="username">"User:"</label>
                    <input class="text" r#type="text" name="username" id="username"/>
                    <label r#for="password">"Pass:"</label>
                    <input class="text" r#type="password" name="password" id="password"/>
                    <input class="button" r#type="submit" name="submit" id="submit" value="Login"/>
                </form>
                <span class="extra_links">
                    "|"
                    <a id="reg_link" href="javascript:">"Register"</a>
                    {allow_password_reset.then(|| view! {
                        <a id="forgot_link" href="javascript:">"Forgot Password"</a>
                    })}
                    <a id="chem_link" href="javascript:" style="display: none;">"Change Email"</a>
                    <a id="chpw_link" href="javascript:" style="display: none;">"Change Password"</a>
                    <a id="logout_link" href="javascript:" style="display: none;">"Logout"</a>
                </span>
            </div>
        </div>
    }
}

#[component]
fn RegisterDialog() -> impl IntoView {
    view! {
        <div id="register" class="floating_dialog" style="display: none;">
            <span id="register_message"></span>
            <form action="#" id="register_form">
                <label r#for="username">"Username:"</label>
                <input class="text" r#type="text" name="username" id="reg_username"/>
                <br/>
                <label r#for="reg_password">"Password:"</label>
                <input class="text" r#type="password" name="reg_password" id="reg_password"/>
                <br/>
                <label r#for="reg_repeat_password">"Repeat password:"</label>
                <input class="text" r#type="password" name="reg_repeat_password" id="reg_repeat_password"/>
                <br/>
                <label r#for="reg_email">"Email address:"</label>
                <input class="text" r#type="text" name="reg_email" id="reg_email"/>
                <br/>
                <p>"If you do not enter an email, you can't recover your password."</p>
                <input class="button" r#type="button" name="cancel" id="reg_cancel" value="Cancel"/>
                <input class="button" r#type="submit" name="submit" id="reg_submit" value="Submit"/>
            </form>
        </div>
    }
}

#[component]
fn ChangePasswordDialog() -> impl IntoView {
    view! {
        <div id="change_password" class="floating_dialog" style="display: none;">
            <span id="chpw_message"></span>
            <form action="#" id="chpw_form">
                <label r#for="change_cur_password">"Current password:"</label>
                <input class="text" r#type="password" name="change_cur_password" id="change_cur_password"/>
                <br/>
                <label r#for="change_password">"New password:"</label>
                <input class="text" r#type="password" name="change_new_password" id="change_new_password"/>
                <br/>
                <label r#for="change_repeat_password">"Repeat password:"</label>
                <input class="text" r#type="password" name="change_repeat_password" id="change_repeat_password"/>
                <br/>
                <br/>
                <input class="button" r#type="button" name="cancel" id="chpw_cancel" value="Cancel"/>
                <input class="button" r#type="submit" name="submit" id="chpw_submit" value="Submit"/>
            </form>
        </div>
    }
}

#[component]
fn ChangeEmailDialog() -> impl IntoView {
    view! {
        <div id="change_email" class="floating_dialog" style="display: none;">
            <p>"Your current email address is: "<span id="chem_current"></span></p>
            <span id="chem_message"></span>
            <form action="#" id="chem_form">
                <label r#for="chem_email">"Email:"</label>
                <input class="text" r#type="text" name="chem_email" id="chem_email"/>
                <br/>
                <p>"If you do not enter an email, you can't recover your password."</p>
                <input class="button" r#type="button" name="cancel" id="chem_cancel" value="Cancel"/>
                <input class="button" r#type="submit" name="submit" id="chem_submit" value="Submit"/>
            </form>
        </div>
    }
}

#[component]
fn ForgotPasswordDialogs() -> impl IntoView {
    view! {
        <div id="forgot" class="floating_dialog" style="display: none;">
            <span id="forgot_message"></span>
            <form action="#" id="forgot_form">
                <label r#for="forgot_email">"Email address:"</label>
                <input class="text" r#type="text" name="forgot_email" id="forgot_email"/>
                <br/>
                <input class="button" r#type="button" name="cancel" id="forgot_cancel" value="Cancel"/>
                <input class="button" r#type="submit" name="submit" id="forgot_submit" value="Submit"/>
            </form>
        </div>

        <div id="forgot_2" class="floating_dialog" style="display: none;">
            <span>"If a matching account was found, then an email was sent to reset your password."</span>
            <input class="button" r#type="submit" name="submit" value="OK"/>
        </div>
    }
}

#[component]
fn ResetPasswordDialog(token: String, error: Option<String>) -> impl IntoView {
    view! {
        <div id="reset_pw" class="floating_dialog" style="display: none;">
            {match error {
                Some(error) => view! {
                    <form action="#" id="reset_pw_form">
                        <span>{error}</span>
                        <br/>
                        <br/>
                        <input class="button" r#type="button" name="cancel" id="reset_pw_cancel" value="Cancel"/>
                    </form>
                }.into_any(),
                None => view! {
                    <span>"Please choose a new password."</span>
                    <br/>
                    <span id="reset_pw_message"></span>
                    <form action="#" id="reset_pw_form">
                        <input class="text" r#type="hidden" readonly="readonly" name="reset_pw_token"
                               id="reset_pw_token" value=token/>
                        <br/>
                        <label r#for="reset_pw_password">"Password:"</label>
                        <input class="text" r#type="password" name="reset_pw_password" id="reset_pw_password"/>
                        <br/>
                        <label r#for="reset_pw_repeat_password">"Repeat password:"</label>
                        <input class="text" r#type="password" name="reset_pw_repeat_password" id="reset_pw_repeat_password"/>
                        <br/>
                        <input class="button" r#type="button" name="cancel" id="reset_pw_cancel" value="Cancel"/>
                        <input class="button" r#type="submit" name="submit" id="reset_pw_submit" value="Submit"/>
                    </form>
                }.into_any(),
            }}
        </div>
    }
}

#[component]
fn RcEditDialog() -> impl IntoView {
    view! {
        <div id="rc_edit" class="floating_dialog" style="display: none;">
            <form action="#" id="rc_edit_form">
                <textarea class="text" name="rc_file_contents" id="rc_file_contents" cols="80" rows="25"></textarea>
                <br/>
                <input class="button" r#type="submit" name="submit" id="rc_submit" value="Save" style="float: right;"/>
            </form>
        </div>
    }
}

#[component]
fn ExitGameDialog() -> impl IntoView {
    view! {
        <div id="exit_game" class="floating_dialog" style="display: none;">
            <p id="exit_game_reason"></p>
            <pre id="exit_game_message"></pre>
            <p id="exit_game_dump"></p>
            <a class="hide_dialog" href="javascript:">"Close"</a>
        </div>
    }
}

#[component]
fn AdminPanel(admin_password_reset: bool) -> impl IntoView {
    view! {
        <div>
            <div id="admin_panel_button" style="display:none;">
                <span><a href="javascript:">"Toggle admin panel"</a></span>
            </div>
            <div id="admin_panel" style="display: none;">
                <div><span><b>"Admin panel"</b></span></div>
                <hr/>
                <div id="admin_announcements">
                    <span>"Enter text to broadcast to all players. Please do not use this frivolously!"</span>
                    <form>
                        <label r#for="announcement_text">"Announcement: "</label>
                        <input class="text" r#type="text" name="announcement_text" id="announcement_text"/>
                        <span><a href="javascript:" id="announcement_submit">"send announcement"</a></span>
                    </form>
                </div>
                <hr/>
                {admin_password_reset.then(|| view! {
                    <div id="user_control">
                        <form action="#" id="admin_user_control">
                            <label r#for="admin_username">"User:"</label>
                            <input class="text" r#type="text" name="admin_username" id="admin_username"/>
                            <input class="button" r#type="button" name="admin_pw_reset" id="admin_pw_reset"
                                   value="Generate password token"/>
                            <input class="button" r#type="button" name="admin_pw_reset_clear" id="admin_pw_reset_clear"
                                   value="Clear password token"/>
                        </form>
                    </div>
                    <hr/>
                })}
                <div id="admin_panel_log"></div>
            </div>
        </div>
    }
}

#[component]
fn LobbyBody() -> impl IntoView {
    view! {
        <div id="lobby_body">
            <span>"Games currently running:"</span>
            <table id="player_list" class="no_game_times">
                <thead>
                    <tr>
                        <th class="username">"User"</th>
                        <th class="game_id">"Game"</th>
                        <th class="xl">"XL"</th>
                        <th class="char">"Char"</th>
                        <th class="place">"Place"</th>
                        <th class="turn">"Turn"</th>
                        <th class="dur">"Time"</th>
                        <th class="god">"God"</th>
                        <th class="idle_time">"Idle"</th>
                        <th class="spectator_count">"Specs"</th>
                        <th class="milestone_col">"Milestone"</th>
                    </tr>
                </thead>
                <tbody></tbody>
            </table>
            <div style="display: none">
                <table>
                    <tr id="game_entry_template">
                        <td class="username"></td>
                        <td class="game_id"></td>
                        <td class="xl"></td>
                        <td class="char"></td>
                        <td class="place"></td>
                        <td class="turn"></td>
                        <td class="dur"></td>
                        <td class="god"></td>
                        <td class="idle_time"></td>
                        <td class="spectator_count"></td>
                        <td class="milestone_col">
                            <div class="milestone_container">
                                <div class="milestone">{"\u{a0}"}</div>
                                <div class="new_milestone"></div>
                            </div>
                        </td>
                    </tr>
                </table>
            </div>
        </div>
    }
}

/// Loader-screen title-art gallery. `static_url("title_....png")` in the old
/// template just became `/static/title_....png`.
const LOADER_IMAGES: &[&str] = &[
    "title_anon_octopus_wizard.png",
    "title_arbituhhh_tesu.png",
    "title_baconkid_duvessa_dowan.png",
    "title_baconkid_gastronok.png",
    "title_baconkid_mnoleg.png",
    "title_benadryl_antaeus.png",
    "title_benadryl_oni.png",
    "title_Cws_Minotauros.png",
    "title_denzi_dragon.png",
    "title_denzi_evil_mage.png",
    "title_denzi_invasion.png",
    "title_denzi_kitchen_duty.png",
    "title_denzi_summoner.png",
    "title_denzi_undead_warrior.png",
    "title_e_m_fields.png",
    "title_firemage.png",
    "title_froggy_goodgod_tengu_gold.png",
    "title_froggy_jiyva_felid.png",
    "title_froggy_natasha_and_boris.png",
    "title_froggy_rune_and_run_failed_on_dis.png",
    "title_froggy_thunder_fist_nikola.png",
    "title_gompami_kohu_xbow.png",
    "title_kaonedong_ignis_the_dying_flame.png",
    "title_kaonedong_menkaure_prince_of_dust.png",
    "title_king7artist_eustachio.png",
    "title_lemurrobot_gozag_vaults.png",
    "title_micah_c_ereshkigal.png",
    "title_nibiki_octopode.png",
    "title_omndra_zot_demon.png",
    "title_peileppe_bloax_eye.png",
    "title_ploomutoo_ijyb.png",
    "title_philosopheropposite_palentonga_paladin.png",
    "title_pooryurik_knight.png",
    "title_psiweapon_kiku.png",
    "title_psiweapon_roxanne.png",
    "title_sastrei_chei.png",
    "title_shadyamish_octm.png",
    "title_SpinningBird_djinn_sears_gnolls.png",
    "title_white_noise_entering_the_dungeon.png",
    "title_white_noise_grabbing_the_orb.png",
    "title_ylam_formicid_shrikes.png",
];

#[component]
fn LoaderImages() -> impl IntoView {
    LOADER_IMAGES
        .iter()
        .map(|name| {
            view! {
                <img style="display:none;" alt="" loading="lazy" src=format!("/static/{name}")/>
            }
        })
        .collect_view()
}

#[component]
fn ChatWidget() -> impl IntoView {
    view! {
        <div id="chat" style="display: none;">
            <a href="javascript:" id="chat_hide_button">
                <span id="chat_hide_button_span">"(-)"</span>
            </a>
            <a href="javascript:" id="chat_caption">
                <span id="spectator_count">"0 spectators"</span>
                <span id="message_count">"0 new messages"</span>
            </a>

            <div id="chat_body" style="display: none;">
                <span id="spectator_list">{"\u{a0}"}</span>

                <div id="chat_history_container">
                    <span id="chat_history"></span>
                </div>

                <input class="text" r#type="text" name="chat_input" id="chat_input" style="display: none"/>
                <div id="chat_login_text">
                    <a id="chat_login_link" href="javascript:">"Login"</a>
                    " to chat"
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every id `assets/static/scripts/{client,chat}.js` (untouched, hard
    /// dependency of the external `game.html`/`game.js` - see module docs)
    /// select on via `$("#...")`/`$('#...')`. Rendered with every optional
    /// section enabled so the whole page is present. This is the only
    /// safety net against silently breaking that untouched-JS contract.
    const REQUIRED_IDS: &[&str] = &[
        "lobby", "login", "login_message", "login_form", "username", "password", "submit", "reg_link",
        "forgot_link", "chem_link", "chpw_link", "logout_link", "register", "register_message", "register_form",
        "reg_username", "reg_password", "reg_repeat_password", "reg_email", "reg_cancel", "reg_submit",
        "change_password", "chpw_message", "chpw_form", "change_cur_password", "change_new_password",
        "change_repeat_password", "chpw_cancel", "chpw_submit", "change_email", "chem_current", "chem_message",
        "chem_form", "chem_email", "chem_cancel", "chem_submit", "floating_ok_message", "ok_message_content",
        "forgot", "forgot_message", "forgot_form", "forgot_email", "forgot_cancel", "forgot_submit", "forgot_2",
        "reset_pw", "reset_pw_cancel", "reset_pw_message", "reset_pw_form", "reset_pw_token", "reset_pw_password",
        "reset_pw_repeat_password", "reset_pw_submit", "rc_edit", "rc_edit_form", "rc_file_contents", "rc_submit",
        "exit_game", "exit_game_reason", "exit_game_message", "exit_game_dump", "banner", "account_restricted",
        "admin_panel_button", "admin_panel", "admin_announcements", "announcement_text", "announcement_submit",
        "user_control", "admin_username", "admin_pw_reset", "admin_pw_reset_clear", "admin_panel_log", "play_now",
        "lobby_body", "player_list", "game_entry_template", "footer", "game", "crt", "loader", "loader_center",
        "loader_text", "stale_processes_message", "force_terminate", "force_terminate_no", "force_terminate_yes",
        "overlay", "chat_hidden", "chat", "chat_hide_button", "chat_hide_button_span", "chat_caption",
        "spectator_count", "message_count", "chat_body", "spectator_list", "chat_history_container", "chat_history",
        "chat_input", "chat_login_text", "chat_login_link", "prompt", "prompt_title", "prompt_footer",
    ];

    #[test]
    fn every_id_the_untouched_js_selects_on_is_present() {
        let html = view! {
            <LobbyPage
                socket_server="ws://x/socket".to_string()
                game_version="0.34".to_string()
                allow_password_reset=true
                admin_password_reset=true
                reset_token=Some("tok".to_string())
                reset_token_error=None
            />
        }
        .to_html();

        for id in REQUIRED_IDS {
            let needle = format!("id=\"{id}\"");
            assert!(html.contains(&needle), "missing {needle} in rendered lobby page");
        }
    }

    #[test]
    fn optional_sections_are_absent_when_disabled() {
        let html = view! {
            <LobbyPage
                socket_server="ws://x/socket".to_string()
                game_version="0.34".to_string()
                allow_password_reset=false
                admin_password_reset=false
                reset_token=None
                reset_token_error=None
            />
        }
        .to_html();

        assert!(!html.contains(r#"id="forgot_link""#));
        assert!(!html.contains(r#"id="forgot""#));
        assert!(!html.contains(r#"id="reset_pw""#));
        assert!(!html.contains(r#"id="user_control""#));
    }

    #[test]
    fn socket_server_and_game_version_are_substituted() {
        let html = view! {
            <LobbyPage
                socket_server="ws://example/socket".to_string()
                game_version="1.2.3".to_string()
                allow_password_reset=false
                admin_password_reset=false
                reset_token=None
                reset_token_error=None
            />
        }
        .to_html();

        assert!(html.contains(r#"var socket_server = "ws://example/socket";"#));
        assert!(html.contains(r#"var game_version = "1.2.3";"#));
    }

    /// Regression test: RequireJS auto-bootstraps `app.js` (which in turn
    /// loads `client`/`comm`/`chat`/`key_conversion`) by reading the
    /// `data-main` attribute off its own `<script>` tag. If this attribute
    /// isn't actually present in the rendered output, nothing ever loads
    /// and the page hangs on the "Loading..." screen forever.
    #[test]
    fn require_js_data_main_bootstrap_attribute_is_present() {
        let html = view! {
            <LobbyPage
                socket_server="ws://x/socket".to_string()
                game_version="0.34".to_string()
                allow_password_reset=false
                admin_password_reset=false
                reset_token=None
                reset_token_error=None
            />
        }
        .to_html();

        assert!(html.contains(r#"data-main="/static/scripts/app""#));
    }
}
