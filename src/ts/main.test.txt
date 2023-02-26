import { describe, it, expect } from '@jest/globals';
import { Main } from './main';

describe('Main', () => {
  it('should work', () => {
    expect(new Main().execute()).toBe(1);
  });
});
