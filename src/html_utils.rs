// Helper function to generate the HTML header.
pub fn html_header() -> String {
    let header = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>WesleySoto.dev</title>
        <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.1.0/github-markdown.min.css">
        <style>
        body { display: flex; margin: 0; height: 100%; }
        .sidebar {
            position: fixed; /* Keeps the sidebar fixed during scrolling */
            top: 0;
            left: 0;
            width: 240px; /* Adjusted width to match the padding of the content */
            height: 100%;
            background-color: var(--color-canvas-subtle);
            overflow-y: auto;
            border-right: 1px solid var(--color-border-muted);
            padding: 20px;
            box-sizing: border-box; /* Includes padding in the width */
        }
        .sidebar a { 
            text-decoration: none; 
            color: inherit;
        }
        .content {
            flex: 1;
            overflow-y: auto;
            padding-left: 260px; /* Added padding to push content away from the sidebar */
            box-sizing: border-box; /* Includes padding in the width calculation */
        }
        .markdown-body {
            max-width: none;
            margin: 0;
            padding: 20px;
        }
        @media (prefers-color-scheme: dark) {
            .sidebar { border-right: 1px solid #444; }
            .sidebar a { color: #58a6ff; }
        }
            /* Additional styles for the sidebar, integrated with github-markdown-css */
            .sidebar {
                position: fixed;
                top: 0;
                left: 0;
                width: 250px;
                height: 100%;
                background-color: var(--color-canvas-subtle); /* Respects GitHub's color scheme */
                overflow-y: auto;
                border-right: 1px solid var(--color-border-muted);
            }

            /* Adjustments for the sidebar links to match GitHub's styles */
            .nav-link {
                color: var(--color-fg-default);
                text-decoration: none;
                display: block;
                padding: 6px 12px;
                line-height: 1.25;
            }

            .nav-link:hover, .nav-link:focus {
                background-color: var(--color-neutral-muted);
                text-decoration: none;
            }

            /* Style for the unordered list in the sidebar */
            .list-style-none {
                list-style: none;
                padding: 0;
                margin: 0;
            }
        </style>
    </head>
    <body class="markdown-body">
        
        <main class="content, markdown-body">
            <!-- The main content goes here -->
        </main>
    </body>
    </html>
    "#;
    header.to_string()
}