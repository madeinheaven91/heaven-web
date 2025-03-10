import axios from "axios";
import Cookies from "js-cookie";
import type { Post, Tag, User } from "./models";

async function delete_post(slug: string) {
  try {
    const response = await axios
      .delete(`http://localhost:8000/api/v1/blog/posts/${slug}`, {
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
  try {
    let profile = await axios
      .get("http://localhost:8000/api/v1/users/profile", {
        headers: {
          Authorization: `Bearer ${localStorage.getItem("access_token")}`,
        },
      })
      .then((res) => res.data);
    return profile;
  } catch (error) {
    try{
      let { data } = await axios
        .get("http://localhost:8000/api/v1/users/refresh", {
          withCredentials: true
        });
      let profile = await axios
        .get("http://localhost:8000/api/v1/users/profile", {
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
      "http://localhost:8000/api/v1/users/login",
      "name=" + name + "&password=" + password,
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
    .get("http://localhost:8000/api/v1/blog/posts/fetch")
    .then((res) => res.data);
  return posts;
}

async function fetch_tags(): Promise<Tag[]> {
  let posts = await axios
    .get("http://localhost:8000/api/v1/blog/tags/fetch")
    .then((res) => res.data);
  return posts;
}

async function fetch_post(slug: string): Promise<Post | null> {
  try {
    let post = await axios
      .get(`http://localhost:8000/api/v1/blog/posts/fetch/${slug}`)
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
