import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import InputField from './components/InputField';
import { Todo } from './components/model';
import { isDefaultClause } from 'typescript';



const App:React.FC = () => {
  const [todo, setTodo] = useState<string>("");
  const [todos, setTodos] = useState<Todo[]>([])

  const handleAdd = (e: React.FormEvent) => {
    e.preventDefault();

    if(todo) {
      setTodos([...todos, {id: Date.now(), todo:todo, isDone:false}])
      setTodo("")
    }
  }

  console.log(todos);
  

  return (
    <div className="App">
    <span className="heading">Todo List</span>
    <InputField todo={todo} setTodo={setTodo} handleAdd={handleAdd}/>
    {/* <TodoList/> */}
    </div>
  );
}

export default App;
