// Execute <script src="foo.js" /> wrapped by <div class="script"> in JSX.
// Wrapping is to detect a position to execute the script.
function useScriptExecution (__html) {
  const [view, setView] = useState<HTMLDivElement | null>(null);

  useEffect(() => {
    const sid = setTimeout(() => {
      view && view.querySelectorAll('.script').forEach((node) => {
        const script = node.querySelector('script');
        if (!script) {
          return;
        }
        const tag = document.createElement('script');
        tag.setAttribute('src', script.getAttribute('src') || '');
        node.appendChild(tag);
      });
    });

    return () => clearTimeout(sid);
  }, [view, __html]);

  return setView
}
