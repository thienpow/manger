use sycamore::prelude::*;

#[component]
pub fn PWAInstall<G: Html>(cx: Scope) -> View<G> {

    view! { cx,
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
                    deferredPrompt = e;
                    installBtn.style.display = 'block';
                    pwaCard.style.display = 'block';

                    installBtn.addEventListener('click', (e) => {
                        installBtn.style.display = 'none';
                        deferredPrompt.prompt();
                        
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
        
    }
}
