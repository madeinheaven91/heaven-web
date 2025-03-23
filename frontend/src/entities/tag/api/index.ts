import { api, api_protected } from "@/shared/api";

export async function fetch_tags() {
  return (await api.get('/blog/tags/fetch')).data;
}

export async function delete_tag(slug: string) {
  try {
    const response = (await api_protected.delete(`/blog/posts/${slug}`)).data;
    return response;
  } catch (error) {
    console.error("Error deleting post:", error);
    return null;
  }
}
