import { AuthClient } from '@dfinity/auth-client';
import { ToastContainer, toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

import { useEffect, useState } from 'react';
import './App.css';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import {
  faTrash,
  faArrowDown,
  faArrowUp,
  faRefresh,
} from '@fortawesome/free-solid-svg-icons';

import { canisterId, createActor, backend } from './declarations/backend';

interface Post {
  id: number;
  content: string;
  votes: number;
}

type Vote = { Up: null } | { Down: null };

function App() {
  const [posts, setPosts] = useState<Post[]>([]);
  const [content, setContent] = useState<string>('');
  const [authClient, setAuthClient] = useState<AuthClient | undefined>();
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [backendActor, setBackendActor] = useState<any>(backend);

  useEffect(() => {
    backend
      .is_anonymous_allowed()
      .then((allowed) => {
        if (allowed) {
          setIsAuthenticated(true);
        }
      })
      .catch((err) => console.error(err));
  }, []);

  const options = {
    createOptions: {
      idleOptions: {
        // Set to true if you do not want idle functionality
        disableIdle: true,
      },
    },

    loginOptions: {
      identityProvider:
        process.env.DFX_NETWORK === 'local'
          ? `http://localhost:4943?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}#authorize`
          : 'https://identity.ic0.app/#authorize',
    },
  };

  const login = () => {
    if (authClient) {
      authClient.login({
        ...options.loginOptions,
        onSuccess: () => {
          updateClient(authClient);
        },
      });
    }
  };

  async function logout() {
    await authClient?.logout();
    if (authClient) {
      await updateClient(authClient);
    }
  }

  const fetchPosts = async () => {
    try {
      const posts = await backendActor.get_posts();
      setPosts(posts);
    } catch (err) {
      console.error(err);
    }
  };

  const newPost = async () => {
    try {
      if (!isAuthenticated) throw new Error('Not authenticated');
      // Not a reliable way to generate unique ids
      // In a real app, you'd use a UUID or generate the id in the canister
      const id = Math.floor(Math.random() * 1000);
      const post: Post = {
        id,
        content: content,
        votes: 0,
      };
      await backendActor.insert(post);
      const updatedPosts = [...posts, post];
      setPosts(updatedPosts);
    } catch (err) {
      console.error(err);
      if (!isAuthenticated) {
        notify('NOT_AUTHENTICATED');
      }
    }
  };

  const handleRemove = async (postId: number) => {
    try {
      await backendActor.remove(postId);
      const updatedPosts = posts.filter((post) => post.id !== postId);
      setPosts(updatedPosts);
    } catch (err) {
      console.error(err);
      if (!isAuthenticated) {
        notify('NOT_AUTHENTICATED');
      }
    }
  };

  useEffect(() => {
    fetchPosts();
    AuthClient.create().then(async (client) => {
      updateClient(client);
    });
  }, []);

  const handleVote = async (postId: number, vote: Vote) => {
    try {

      if (!isAuthenticated) throw new Error('Not authenticated');
      let updated_post = await backendActor.vote(postId, vote);

      setPosts(
        posts.map((post) => {
          if (post.id === postId) {
            return { ...post, updated_post };
          }
          return post;
        }),
      );
      fetchPosts();
    } catch (err) {
      console.log(err);
      if (!isAuthenticated) {
        notify('NOT_AUTHENTICATED');
      } else {
        notify('ALREADY_VOTED');
      }
    }
  };

  async function updateClient(client: AuthClient) {
    const isAuthenticated = await client.isAuthenticated();
    setIsAuthenticated(isAuthenticated);

    const identity = client.getIdentity();

    setAuthClient(client);

    const actor = createActor(canisterId, {
      agentOptions: {
        identity,
      },
    });

    setBackendActor(actor);
  }

  const msg = (TYPE: string) => {
    switch (TYPE) {
      case 'NOT_AUTHENTICATED':
        return 'You are not logged in';
      case 'ALREADY_VOTED':
        return 'You have already voted on this post';
    }
  };

  const notify = (TYPE: string) => {
    toast.error(msg(TYPE), {
      position: toast.POSITION.BOTTOM_CENTER,
    });
  };

  const sortedPosts = [...posts].sort((a, b) => b.votes - a.votes);

  return (
    <div>
      <h1>Reddit Clone</h1>
      <button onClick={fetchPosts}>
        {' '}
        <FontAwesomeIcon icon={faRefresh} /> Refresh Posts
      </button>
      {isAuthenticated ? (
        <button onClick={logout}>
          Logout
        </button>
      ) : (
        <button onClick={login}>
          Login
        </button>
      )}
      <div className="card">
        <input
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="Enter something..."
        />
        <button onClick={newPost}>New Post</button>
      </div>
      <ul>
        {sortedPosts.map((post) => (
          <li key={post.id}>
            <div>{post.content}</div>
            <div>
              <button onClick={() => handleVote(post.id, { Up: null })}>
                <FontAwesomeIcon icon={faArrowUp} />
              </button>
              <span>{post.votes}</span>
              <button onClick={() => handleVote(post.id, { Down: null })}>
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
      <ToastContainer autoClose={2000} />
    </div>
  );
}

export default App;
