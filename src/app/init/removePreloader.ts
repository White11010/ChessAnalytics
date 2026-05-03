export function removePreloader(): void {
  const loader = document.getElementById('preloader');
  if (loader) loader.remove();
}
