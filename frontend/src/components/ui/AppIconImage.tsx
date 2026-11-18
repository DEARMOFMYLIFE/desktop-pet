import type { AppIconImageProps } from '../../types';

export function AppIconImage({ src, className }: AppIconImageProps) {
  return <img className={className} src={src} alt="" decoding="async" />;
}
