cd backend

cargo run &

cd ../frontend

npm run dev &

echo "Press Ctrl+C to stop the server"
echo "Backend is running on port 3000"
echo "Frontend is running on port 5173" 


wait