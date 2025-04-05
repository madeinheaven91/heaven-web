import { api, api_protected } from "@/shared/api";

export async function fetch_posts() {
  return (await api.get('/blog/posts/fetch')).data;
}

export async function fetch_post(slug: string) {
  return (await api.get(`/blog/posts/fetch/${slug}`)).data;
}

export async function delete_post(slug: string) {
  try {
    const response = (await api_protected.delete(`/blog/posts/${slug}`)).data;
    return response;
  } catch (error) {
    console.error("Error deleting post:", error);
    return null;
  }
}

export async function publish(slug: string) {
  return (await api_protected.patch(`/blog/posts/${slug}`, {
    is_published: true
  })).data;
}
