/**
 * Example: Using f2v2f TypeScript binding directly
 */

import { Encoder, Decoder, version, F2V2FError } from './bindings/typescript';
import fs from 'fs/promises';
import path from 'path';

async function main(): Promise<number> {
  try {
    console.log(`f2v2f version: ${version()}`);
    console.log();

    // Create a test file
    const testFile = path.join(__dirname, 'example_input.txt');
    const outputVideo = path.join(__dirname, 'example_output.mp4');
    const recoveredFile = path.join(__dirname, 'example_recovered.txt');

    await fs.writeFile(testFile, 'Hello, World! This is a test file for f2v2f encoding.');

    try {
      // Encoding example
      console.log('📹 Encoding file to video...');
      const encoder = new Encoder(1920, 1080, 30, 65536);

      const progressCallback = (totalBytes: number, totalFrames: number, message: string) => {
        console.log(`   Progress: ${totalFrames} frames, ${totalBytes} bytes - ${message}`);
      };

      await encoder.encode(testFile, outputVideo, progressCallback);
      console.log(`✅ Encoding complete! Video: ${outputVideo}`);
      console.log();

      // Decoding example
      console.log('🎬 Decoding video back to file...');
      const decoder = new Decoder();
      await decoder.decode(outputVideo, recoveredFile, progressCallback);
      console.log(`✅ Decoding complete! File: ${recoveredFile}`);
      console.log();

      // Verify
      const originalContent = await fs.readFile(testFile, 'utf-8');
      const recoveredContent = await fs.readFile(recoveredFile, 'utf-8');

      if (originalContent === recoveredContent) {
        console.log('✅ Verification: Files match perfectly!');
      } else {
        console.log('❌ Verification failed: Files don\'t match');
        return 1;
      }

      encoder.destroy();
      decoder.destroy();

    } catch (error) {
      if (error instanceof F2V2FError) {
        console.error(`❌ F2V2F error: ${error.message}`);
      } else {
        console.error(`❌ Error: ${error}`);
      }
      return 1;
    }

    return 0;

  } finally {
    // Cleanup
    const testFile = path.join(__dirname, 'example_input.txt');
    const outputVideo = path.join(__dirname, 'example_output.mp4');
    const recoveredFile = path.join(__dirname, 'example_recovered.txt');

    try {
      await fs.unlink(testFile);
    } catch { }
    try {
      await fs.unlink(outputVideo);
    } catch { }
    try {
      await fs.unlink(recoveredFile);
    } catch { }
  }
}

main().then(code => process.exit(code)).catch(err => {
  console.error('Fatal error:', err);
  process.exit(1);
});
