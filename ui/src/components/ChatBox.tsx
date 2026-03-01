import { useState, useRef, useEffect } from "react";
import type { AnalysisResult } from "../types";

type Message = {
  id: number;
  role: "user" | "bot";
  text: string;
};

interface Props {
  result: AnalysisResult | null;
}

const suggestions = [
  "Find unused functions",
  "List all functions",
  "List all classes",
  "Check dependencies",
];

const CHAT_URL = "http://localhost:8080/chat";

export default function ChatBox({ result }: Props) {
  const [messages, setMessages] = useState<Message[]>([
    { id: 0, role: "bot", text: "Hi! Ask me anything about your code." }
  ]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const [conversationId, setConversationId] = useState<string | null>(null);
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [messages]);

  // Reset conversation when analysis result changes (new file/repo analyzed)
  useEffect(() => {
    setConversationId(null);
    setMessages([{ id: 0, role: "bot", text: "Hi! Ask me anything about your code." }]);
  }, [result]);

  const send = async (text: string) => {
    if (!text.trim() || loading) return;

    if (!result) {
      setMessages(prev => [...prev,
        { id: Date.now(), role: "user", text },
        { id: Date.now() + 1, role: "bot", text: "Please run an analysis first before asking questions." }
      ]);
      return;
    }

    setMessages(prev => [...prev, { id: Date.now(), role: "user", text }]);
    setInput("");
    setLoading(true);

    try {
      const body: Record<string, unknown> = {
        message: text,
        conversation_id: conversationId,
      };

      // Pass analysis result as context on first message only
      if (conversationId === null) {
        body.context = result;
      }

      const resp = await fetch(CHAT_URL, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
      });

      if (!resp.ok) {
        const err = await resp.json().catch(() => ({ detail: resp.statusText }));
        throw new Error(err.detail ?? `Server error ${resp.status}`);
      }

      const data = await resp.json();

      if (conversationId === null) {
        setConversationId(data.conversation_id);
      }

      setMessages(prev => [...prev, { id: Date.now() + 1, role: "bot", text: data.reply }]);

    } catch (err) {
      setMessages(prev => [...prev, {
        id: Date.now() + 1,
        role: "bot",
        text: `Error: ${err instanceof Error ? err.message : String(err)}`,
      }]);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="chatbox">
      <h3>Ask About Code</h3>

      <div className="chat-messages">
        {messages.map(m => (
          <div key={m.id} className={`chat-bubble ${m.role}`}>
            {m.text}
          </div>
        ))}
        {loading && <div className="chat-bubble bot">Thinking…</div>}
        <div ref={bottomRef} />
      </div>

      <div className="chat-suggestions">
        {suggestions.map(s => (
          <button key={s} className="chat-suggestion" onClick={() => send(s)} disabled={loading || !result}>
            {s}
          </button>
        ))}
      </div>

      <div className="chat-input-row">
        <input
          className="chat-input"
          type="text"
          placeholder={result ? "Ask about your code…" : "Run analysis first…"}
          value={input}
          onChange={e => setInput(e.target.value)}
          onKeyDown={e => e.key === "Enter" && send(input)}
          disabled={loading}
        />
        <button className="chat-send" onClick={() => send(input)} disabled={loading || !result}>↑</button>
      </div>
    </div>
  );
}