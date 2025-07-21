/**
 * Processes the content of a post by ensuring that text elements fit within specified dimensions.
 * 
 * If a text element's height exceeds the allowed maximum, it is recursively split at the nearest 
 * space to the center and the resulting parts are appended as new elements to the DOM. Non-text 
 * elements like code blocks, audio, and video are ignored.
 * 
 * @param {string} submissionId - The unique ID of the post submission to process.
 * @param {Object} dimensions - The original dimensions (width and height) of the container.
 * @param {number} dimensions.width - The original width of the container.
 * @param {number} dimensions.height - The original height of the container.
 * 
 * @returns {Array<Object>} An array of metadata objects for the processed text elements.
 * Each object contains:
 *   - `selector`: A unique CSS selector for the element.
 *   - `text`: The text content of the element.
 *   - **Optional** `separator` {string}: Specifies the type of separation between consecutive elements.
 *      - `"whitenoise"`: Adds audio gaps before and after the blockquote to distinguish it.
 *      - `"space"`: Adds space or silence to separate `<p>` elements for readability.
 *      - `"none"`: No additional separation.
 */
function processPostContent(submissionId, dimensions) {
    const POST_CONTENT_SELECTOR = `#${submissionId}-post-rtjson-content`;
    const POST_CONTENT_SELECTOR_CHILDREN = `${POST_CONTENT_SELECTOR} > *`;
    const ADJUSTMENT = 0.75;

    // Adjust the maximum dimensions (75% of the original width/height)
    const {
        maxWidth,
        maxHeight
    } = {
        maxWidth: dimensions.width * ADJUSTMENT,
        maxHeight: dimensions.height * ADJUSTMENT
    };

    const proccessIndividualElement = (element, prefixSelector) => {
        const tagName = element.tagName.toLowerCase();

        // TODO - shreddit-spoiler

        // Skip certain elements like code blocks, audio, and video
        // Ignore figure due to copyrights
        if (["pre", "audio", "video", "figure","shreddit-spoiler"].includes(tagName)) {
            return;
        }

        // TODO - FOR NOW Skip non-text elements like lists, tables
        if (["ol", "ul", "table"].includes(tagName)) {
            return;
        }

        const processedElementMetadata = [];
        const getTextContent = (element) => element.innerText || element.textContent;

        const proccessElementContainingText = (element) => {

            const textContent = getTextContent(element);

            // Set the element's width to the max width for consistent rendering
            element.style.maxWidth = `${maxWidth}px`;
            element.style.width = `${maxWidth}px`;

            const isOverflowingHeight = (el) => el.offsetHeight > maxHeight;

            const textElements = [element]; // Track all processed elements

            // Handle overflow by recursively splitting text
            if (isOverflowingHeight(element)) {
                const splitTextElement = (element) => {
                    // Helper function: Split text at the nearest space to the center
                    const splitAtCenter = (str) => {
                        const mid = Math.floor(str.length / 2);
                        const leftSpace = str.lastIndexOf(" ", mid);
                        const rightSpace = str.indexOf(" ", mid);

                        let splitIndex;
                        if (leftSpace === -1 && rightSpace === -1) return [str, ""];
                        if (leftSpace === -1) splitIndex = rightSpace;
                        else if (rightSpace === -1) splitIndex = leftSpace;
                        else splitIndex = (mid - leftSpace <= rightSpace - mid) ? leftSpace : rightSpace;

                        return [str.slice(0, splitIndex), str.slice(splitIndex + 1)];
                    };

                    const recursiveElementSplit = (text) => {
                        // Split and append elements until all text fits within the max height
                        while (isOverflowingHeight(element)) {
                            const [left, right] = splitAtCenter(text);
                            element.innerText = left; // Set left part in current element

                            if (isOverflowingHeight(element)) {
                                // Recursively split the left part if it still overflows
                                recursiveElementSplit(left);
                            } else {
                                // Create a new element for the right part if left fits
                                const newElement = document.createElement(tagName);
                                newElement.style.cssText = element.style.cssText; // Copy styles

                                textElements.push(newElement); // Append new element 

                                // Process the right part if it exists
                                if (right) {
                                    recursiveElementSplit(right);
                                }
                            }


                            return; // Exit loop after successful split and append
                        }

                        // If content fits, append the current element to the DOM
                        textElements.push(element);
                    };

                    recursiveElementSplit(textContent);
                };

                splitTextElement(element); // Begin splitting the current element
            }

            const appendElementToPost = (ele) => {
                const post = document.querySelector(POST_CONTENT_SELECTOR);
                post.appendChild(ele);
            };

            // Add metadata for processed elements with unique selectors
            textElements.forEach((ele, innerIndex) => {

                const selector = `${prefixSelector}${tagName}-${innerIndex}`;
                ele.classList.add(selector); // Add a unique class
                appendElementToPost(ele);

                processedElementMetadata.push({
                    selector: selector,
                    text: ele.innerText,
                });
            });
        };

        const proccessAnyTextElement = (element) => {
            // Helper function: Check if the element is a text-based tag
            const isTextElement = () => {
                const textTags = ["h1", "h2", "h3", "h4", "h5", "h6", "p"];
                return textTags.includes(tagName);
            };


            // Handle only text elements
            if (isTextElement()) {
                const previousLen = processedElementMetadata.length();
                proccessElementContainingText(element);

                if (tagName == "p") {
                    for (let i = previousLen; i < processedElementMetadata.length(); i++) {
                        processedElementMetadata[i]["seperator"] = "space"
                    }
                }
                return true
            }

            return false
        }

        if (!proccessAnyTextElement(element)) {
            if (tagName == "blockquote") {
                const blockquoteMetadata = []

                element.children.forEach((element, index) => {
                    const elementMetadata = proccessIndividualElement(element, `${prefixSelector}-${index}`);
                    blockquoteMetadata.push(elementMetadata)
                })

                const altered = blockquoteMetadata.map((metadata) => {
                    metadata["separator"] = "whitenoise";
                    metadata
                });

                processedElementMetadata.push(altered)
            }
        }

        return processedElementMetadata
    };

    const directChildrensOfPost = document.querySelectorAll(POST_CONTENT_SELECTOR_CHILDREN);

    const allElementMetadata = [];

    directChildrensOfPost.forEach((element, index) => {
        const elementMetadata = proccessIndividualElement(element, `parent-${index}`);
        allElementMetadata.push(elementMetadata)
    });

    return allElementMetadata
}