import { useEffect, useState } from 'react';
import './App.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faTrash,
  faArrowDown,
  faArrowUp,
  faRefresh,
} from '@fortawesome/free-solid-svg-icons';

import { backend } from './declarations/backend';

interface Post {
  id: number;
  content: string;
  votes: number;
}

function App() {
  const [posts, setPosts] = useState<Post[]>([]);

  const [content, setContent] = useState<string>('');

  const fetchPosts = async () => {
    try {
      const posts = await backend.get_posts();
      console.log(posts);
      setPosts(posts);
    } catch (err) {
      console.error(err);
    }
  };

  const newPost = async () => {
    try {
      const id = Math.floor(Math.random() * 1000);
      const post: Post = {
        id,
        content: content,
        votes: 0,
      };
      backend.insert(post);
      const updatedPosts = [...posts, post];
      setPosts(updatedPosts);
    } catch (err) {
      console.error(err);
    }
  };

  const handleRemove = async (postId: number) => {
    backend.remove(postId);
    const updatedPosts = posts.filter((post) => post.id !== postId);
    setPosts(updatedPosts);
  };

  useEffect(() => {
    fetchPosts();
  }, []);

  const handleVote = async (postId: number, increment: number) => {
    let updated_post = await backend.vote(postId, increment);
    setPosts(
      posts.map((post) => {
        if (post.id === postId) {
          return { ...post, updated_post };
        }
        return post;
      }),
    );
    fetchPosts();
  };

  return (
    <div>
      <h1>Reddit Clone</h1>
      <button onClick={fetchPosts}>
        {' '}
        <FontAwesomeIcon icon={faRefresh} /> Refresh Posts
      </button>
      <div className="card">
        <input
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="Enter something..."
        />
        <button onClick={newPost}>New Post</button>
      </div>
      <ul>
        {posts.map((post) => (
          <li key={post.id}>
            <div>{post.content}</div>
            <div>
              <button onClick={() => handleVote(post.id, 1)}>
                <FontAwesomeIcon icon={faArrowUp} />
              </button>
              <span>{post.votes}</span>
              <button onClick={() => handleVote(post.id, -1)}>
                <FontAwesomeIcon icon={faArrowDown} />
              </button>
              <button onClick={() => handleRemove(post.id)}>
                {' '}
                <FontAwesomeIcon icon={faTrash} />
              </button>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
