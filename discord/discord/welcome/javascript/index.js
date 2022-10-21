(s) => {
    let message = JSON.parse(s);
    if (message["type"] === 7)          // MemberJoin event
        return JSON.stringify({
            content: "welcome <@" + message["author"]["id"] + ">!",
        })
}
