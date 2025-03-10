interface User {
  id: number;
  name: string;
  email: string;
  is_staff: boolean;
}

interface Post {
  slug: string;
  title: string;
  body: string;
  author: User;
  is_published: boolean;
  created_at: string;
  updated_at: string | undefined;
  tags: Tag[];
}

interface Tag {
  slug: string;
  name: string;
  background_color: string;
  foreground_color: string;
}

export { type User, type Post, type Tag }
