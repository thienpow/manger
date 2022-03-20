use sycamore::prelude::*;
use web_sys::{console, Event};

use crate::{components::{contactbar::ContactBar}, pages::home::sidebar::SidebarLeft};


#[component]
pub fn Home<G: Html>(ctx: Scope) -> View<G> {

    view! { ctx,

        /*
        div(class="main-header") {
            div(class="header-menu") {
                a(class="main-header-link is-active", href="#") { "Desktop" }
                a(class="main-header-link", href="#") { "Mobile" }
                a(class="main-header-link", href="#") { "Web" }
            }
        }
        */
        
        div(class="wrapper") {

            SidebarLeft()

            div(class="main-container") {

                article(class="content-wrapper") {
                    div(class="content-section") {
                        header(class="content-section-title") {
                            "Welcome to use Manger"
                        }
                        div(class="card-wrapper") {
                            div(id="cardPWA", class="card") {
                                span() {"v0.0.1"}
                                div(class="card__subtext") {"You can install this app to your homescreen now!"}
                                div(class="card-buttons") {
                                    button(id="btnPWA", class="cbutton status-button") {"Add to Homescreen"}
                                }

                                script(type="text/javascript") {
                                    r#"
                                    let deferredPrompt;
                                    const installBtn = document.getElementById('btnPWA');
                                    
                                    const pwaCard = document.getElementById('cardPWA');
                                    installBtn.style.display = 'none';
                                    pwaCard.style.display = 'none';
                                    
                                    window.addEventListener('beforeinstallprompt', (e) => {
                                        e.preventDefault();
                                        // Stash the event so it can be triggered later.
                                        deferredPrompt = e;
                                        // Update UI to notify the user they can add to home screen
                                        installBtn.style.display = 'block';
                                        pwaCard.style.display = 'block';
                                        console.log(installBtn);

                                        installBtn.addEventListener('click', (e) => {
                                        // hide our user interface that shows our A2HS button
                                        installBtn.style.display = 'none';
                                        // Show the prompt
                                        deferredPrompt.prompt();
                                        // Wait for the user to respond to the prompt
                                        deferredPrompt.userChoice.then((choiceResult) => {
                                            if (choiceResult.outcome === 'accepted') {
                                                console.log('User accepted the A2HS prompt');
                                            } else {
                                                console.log('User dismissed the A2HS prompt');
                                            }
                                            deferredPrompt = null;
                                            });
                                        });
                                    });
                                    "#
                                }
                            }
                            
                            div(class="card") {
                                span() {"Bible image here"}
                                div(class="card__subtext") {"Please choose a preferred Bible language."}
                                div(class="card-buttons") {
                                    button(class="cbutton status-button") {"中文简体"}
                                    div(class="menu")
                                }
                            }
                        }
                        
                    }
                    
                }
            }

            ContactBar()
        }

                            
    }
}
