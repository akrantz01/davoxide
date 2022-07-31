import { useEffect } from 'react';

const usePageTitle = (title: string): void => {
  useEffect(() => {
    document.title = `DAVOxide - ${title}`;
  }, [title]);
};

export default usePageTitle;
