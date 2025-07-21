function changeTheme(useDarkMode) {
    // Open the user drawer to access the theme toggle
    const userDrawerButton = document.querySelector("#expand-user-drawer-button");
    if (!userDrawerButton) throw new Error("User drawer button not found.");
    userDrawerButton.click();
    
    // Retrieve the current dark mode state (aria-checked returns a string)
    const darkModeToggle = document.querySelector("#darkmode-list-item > div > span.flex.items-center.shrink-0 > span > faceplate-switch-input");
    if (!darkModeToggle) throw new Error("Dark mode toggle not found.");
    
    const isDarkMode = darkModeToggle.ariaChecked === "true";

    // Toggle theme if the current state doesn't match the desired state
    const toggleButton = document.querySelector("#darkmode-list-item > div");
    if (!toggleButton) throw new Error("Theme toggle button not found.");
    
    if (useDarkMode !== isDarkMode) {
        toggleButton.click();
    }
}