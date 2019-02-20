// expect an image for a circle container
export default function calculateImageCenteringStyle (image: HTMLImageElement, container: HTMLElement) {
  const { naturalWidth, naturalHeight } = image;
  const { clientWidth: size } = container;

  return naturalHeight === naturalWidth
    ? {}
    : naturalHeight > naturalWidth
      ? { width: `${size}px`, height: 'auto', marginTop: `${(naturalWidth - naturalHeight) * size / naturalWidth / 2}px` }
      : { width: 'auto', height: `${size}px`, marginLeft: `${(naturalHeight - naturalWidth) * size / naturalHeight / 2}px` };
}
