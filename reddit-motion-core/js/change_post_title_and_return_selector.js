/**
 * Updates the title of a post and wraps it along with the credit bar in a new div container.
 *
 * @param {string} submissionId - The unique ID of the post to be updated.
 * @param {string?} newTitle - The new title to be set for the post.
 * 
 * @returns {string} - The CSS selector for the newly created wrapper div containing the title and credit bar.
 *
 * This function:
 * - Changes the text content of the post's title (`<h1>`).
 * - Wraps the updated title and the existing credit bar (`<div>`) in a new `div` with the class `title-post-wrapper`.
 * - Inserts this wrapper at the beginning of the post.
 */
function changePostTitleAndReturnSelector(submissionId, newTitle) {
    // Define the main post, title, and credit bar selectors using the post ID
    const POST_SELECTOR = `#${submissionId}`;
    const TEXT_SELECTOR = `#post-title-${submissionId}`;
    const CREDIT_BAR_SELECTOR = `${POST_SELECTOR} > div`;
    const NEW_TITLE_CLASS = "title-post-wrapper";
    const NEW_TITLE_SELECTOR = `${POST_SELECTOR}.${NEW_TITLE_CLASS}`;

    if (newTitle != null) {
        // Update the title text in the post
        document.querySelector(TEXT_SELECTOR).textContent = newTitle;
    }

    // Create a new wrapper div and add the class 'title-post-wrapper'
    const divWrapper = document.createElement("div");
    divWrapper.classList.add(NEW_TITLE_CLASS);

    // Append the existing credit bar and title elements into the wrapper
    divWrapper.appendChild(document.querySelector(CREDIT_BAR_SELECTOR));
    divWrapper.appendChild(document.querySelector(TEXT_SELECTOR));

    // Insert the wrapper as the first child of the post element
    const post = document.querySelector(POST_SELECTOR);
    post.insertBefore(divWrapper, post.firstChild);

    // Return the CSS selector for the newly created wrapper
    return NEW_TITLE_SELECTOR;
}
