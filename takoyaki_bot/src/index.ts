import { Probot } from "probot";

export = (app: Probot) => {
    app.on("issues.opened", async (context) => {
        const issueComment = context.issue({
            body: `Thanks for opening this issue, @${context.payload.sender.login}! Your issue will be soon reviewed by the author :)`,
        });

        await context.octokit.issues.createComment(issueComment);
    });

    app.on("issue_comment.created", async (context) => {
        if(context.isBot) return;

        console.log(context.payload)

        const issueComment = context.issue({
            body: "Thanks for opening this issue!",
        });

        await context.octokit.issues.createComment(issueComment);
    });
};
