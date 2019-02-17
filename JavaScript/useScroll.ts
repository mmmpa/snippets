// Scroll to specific position.
// `position` argument is used as a rate (min: 0, max: 1).  
function useScroll (position) {
  const [view, setView] = useState<HTMLDivElement | null>(null);
  useEffect(() => {
    if (!view) {
      return;
    }
    const { scrollHeight, clientHeight } = view;
    view.scrollTo(0, (scrollHeight - clientHeight) * position);
  }, [view, position]);

  return setView;
}
