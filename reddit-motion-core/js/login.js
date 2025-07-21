async function login(username,password) {
    // Helper to enter values into input fields and trigger change event
    const enterFor = (selector,value) => {
        const sendEvent = (element) => {
            var evt = document.createEvent("Events");
            evt.initEvent("change", true, true);
            element.dispatchEvent(evt);        
        }

        const element = document.querySelector(selector)
        if (element) {
            element.value = value;
            sendEvent(element);
        } else {
            throw new Error(`Element with selector "${selector}" not found.`);
        }
    }

    // Enter credentials
    enterFor("#login-username",username) 
    enterFor("#login-password",password) 

    const authentication_parent_element = document.querySelector("body > shreddit-app > shreddit-overlay-display")
        .shadowRoot.querySelector("shreddit-signup-drawer")
        .shadowRoot.querySelector("shreddit-drawer > div > shreddit-async-loader > div > shreddit-slotter");

    const login_element = authentication_parent_element
        .shadowRoot.querySelector("#login > auth-flow-modal > div.w-100 > faceplate-tracker > button");

    login_element.click()

    const error_element = authentication_parent_element
        .shadowRoot.querySelector("#login-password").shadowRoot.querySelector("faceplate-form-helper-text");

    if (error_element.level === "error") {
        throw new Error("Login failed due to incorrect password.");
    }


    // https://stackoverflow.com/a/33292942
    // https://stackoverflow.com/a/74004794
    const delay = (timeMs) => new Promise((resolve) => setTimeout(resolve, timeMs));

    // Wait for 10 seconds and check if redirected
    await delay(10000);

    const DESIRED_URL = 'https://www.reddit.com';

    if (window.location.href !== DESIRED_URL) {
        throw new Error("Not redirected, login failed.");
    }
}