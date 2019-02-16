export async function imageFileToDataUrl (file: File): Promise<string> {
  const reader = new FileReader();
  const p = new Promise<string>((resolve) => {
    // eslint-disable-next-line no-multi-assign
    reader.onload = reader.onerror = function handler (e: FileReaderProgressEvent): void {
      resolve(e.target.result as string);
    };
  });
  reader.readAsDataURL(file);
  return p;
}

type FileReaderProgressEvent = ProgressEvent & {
  target: FileReader
};
