import axios from "axios";
import type { Post, Tag, User } from "./models";

const BASE_URL = import.meta.env.VITE_API_URL;

async function delete_post(slug: string) {
  try {
    const response = await axios
      .delete(`${BASE_URL}/blog/posts/${slug}`, {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("access_token")}`,
        },
        timeout: 10000,
      })
      .then((res) => res.data);
    return response;
  } catch (error) {
    console.error("Error deleting post:", error);
    return null;
  }
}

async function load_profile(): Promise<User | null> {
  console.log(import.meta.env);
  try {
    let profile = await axios
      .get(`${BASE_URL}/users/profile`, {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("access_token")}`,
        },
      })
      .then((res) => res.data);
    return profile;
  } catch (error) {
    try{
      await axios
        .get(`${BASE_URL}/users/refresh`, {
          withCredentials: true
        });
      let profile = await axios
        .get(`${BASE_URL}/users/profile`, {
          headers: {
            Authorization: `Bearer ${localStorage.getItem("access_token")}`,
          },
        })
        .then((res) => res.data);
      return profile;
    }
    catch (error) {
      return null;
    }
  }
}

async function login(name: string, password: string) {
  let resp = await axios
    .post(
      `${BASE_URL}/users/login`,
      `name=${name}&password=${password}`,
      {
        withCredentials: true,
        headers: {
          "Content-Type": "application/x-www-form-urlencoded",
        },
      },
    )
    .then((res) => res.data);
  // localStorage.setItem("access_token", resp.access_token);
  // document.location.reload();
  return resp;
}

function remove_token() {
  localStorage.removeItem("access_token");
  document.location.reload();
}

async function fetch_posts(): Promise<Post[]> {
  let posts = await axios
    .get(`${BASE_URL}/blog/posts/fetch`)
    .then((res) => res.data);
  return posts;
}

async function fetch_tags(): Promise<Tag[]> {
  let posts = await axios
    .get(`${BASE_URL}/blog/tags/fetch`)
    .then((res) => res.data);
  return posts;
}

async function fetch_post(slug: string): Promise<Post | null> {
  try {
    let post = await axios
      .get(`${BASE_URL}/blog/posts/fetch/${slug}`)
      .then((res) => res.data);
    return post;
  } catch (error) {
    return null;
  }
}

export {
  load_profile,
  login,
  remove_token,
  fetch_posts,
  fetch_tags,
  delete_post,
  fetch_post
};
