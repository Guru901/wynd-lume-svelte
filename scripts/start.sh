cd backend

# This binary name should be the name of the package from `Cargo.toml`
./target/release/backend &

cd ../frontend

npm run preview &

echo "Press Ctrl+C to stop the server"
echo "Backend is running on port 3000"
echo "Frontend is running on port 4173"

wait
