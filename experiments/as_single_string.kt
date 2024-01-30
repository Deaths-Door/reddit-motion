fun main() {
    val func = """function() {
        document.querySelector("shreddit-async-loader[bundlename=\"reddit_cookie_banner\"]")?.remove();
        document.querySelector("shreddit-async-loader[bundlename=\"desktop_rpl_nsfw_blocking_modal\"]")?.remove();
        document.querySelector("shreddit-app > div")?.style.filter="none";
    }""".trim().replace("\n","").replace("    ","").replace(""""""","""\"""").replace("""\\"""","""\"""")

    println(func)
}