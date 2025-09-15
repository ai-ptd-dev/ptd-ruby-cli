require 'spec_helper'
require_relative '../../src/utils/database'
require 'tempfile'

RSpec.describe TodoCli::Utils::Database do
  let(:temp_db) { Tempfile.new('test_todos.db') }
  let(:database) { described_class.new }

  before do
    # Mock the database file path to use a temporary file
    stub_const('TodoCli::Utils::Database::DB_FILE', temp_db.path)
  end

  after do
    database.close
    temp_db.unlink
  end

  describe '#add_todo' do
    it 'adds a todo with default priority' do
      id = database.add_todo('Buy groceries')

      expect(id).to be > 0
    end

    it 'adds a todo with specified priority' do
      id = database.add_todo('Urgent task', 'high')
      todo = database.find_todo(id)

      expect(todo['priority']).to eq('high')
      expect(todo['text']).to eq('Urgent task')
    end
  end

  describe '#list_todos' do
    before do
      database.add_todo('High priority task', 'high')
      database.add_todo('Medium priority task', 'medium')
      database.add_todo('Low priority task', 'low')
    end

    it 'returns all pending todos by default' do
      todos = database.list_todos(show_completed: false)

      expect(todos.length).to eq(3)
      expect(todos.all? { |t| t['completed'] == 0 }).to be true
    end

    it 'orders todos by priority' do
      todos = database.list_todos(show_completed: false)

      expect(todos[0]['priority']).to eq('high')
      expect(todos[1]['priority']).to eq('medium')
      expect(todos[2]['priority']).to eq('low')
    end

    it 'includes completed todos when requested' do
      # Complete one todo
      first_todo = database.list_todos(false).first
      database.complete_todo(first_todo['id'])

      all_todos = database.list_todos(show_completed: true)
      pending_todos = database.list_todos(false)

      expect(all_todos.length).to eq(3)
      expect(pending_todos.length).to eq(2)
    end
  end

  describe '#complete_todo' do
    let(:todo_id) { database.add_todo('Test todo') }

    it 'marks todo as completed' do
      result = database.complete_todo(todo_id)
      todo = database.find_todo(todo_id)

      expect(result).to be true
      expect(todo['completed']).to eq(1)
      expect(todo['completed_at']).not_to be_nil
    end

    it 'returns false for non-existent todo' do
      result = database.complete_todo(999)

      expect(result).to be false
    end
  end

  describe '#delete_todo' do
    let(:todo_id) { database.add_todo('Test todo') }

    it 'deletes the todo' do
      result = database.delete_todo(todo_id)
      todo = database.find_todo(todo_id)

      expect(result).to be true
      expect(todo).to be_nil
    end

    it 'returns false for non-existent todo' do
      result = database.delete_todo(999)

      expect(result).to be false
    end
  end

  describe '#find_todo' do
    let(:todo_id) { database.add_todo('Test todo', 'high') }

    it 'finds existing todo' do
      todo = database.find_todo(todo_id)

      expect(todo).not_to be_nil
      expect(todo['text']).to eq('Test todo')
      expect(todo['priority']).to eq('high')
      expect(todo['completed']).to eq(0)
    end

    it 'returns nil for non-existent todo' do
      todo = database.find_todo(999)

      expect(todo).to be_nil
    end
  end
end
