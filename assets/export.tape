# terminal-slide: Export demo
Output assets/export.gif

Set Shell "bash"
Set FontSize 16
Set Width 1200
Set Height 400
Set Padding 20
Set Theme "Catppuccin Mocha"
Set TypingSpeed 50ms

Type "# Export to PDF, HTML, PPTX, Markdown"
Enter
Sleep 1s

Type "terminal-slide examples/demo.md --export html -o /tmp/demo-export.html"
Enter
Sleep 3s

Type "ls -la /tmp/demo-export.html"
Enter
Sleep 2s

Type "terminal-slide examples/demo.md --export pdf -o /tmp/demo-export.pdf"
Enter
Sleep 5s

Type "ls -la /tmp/demo-export.pdf"
Enter
Sleep 2s
